pub struct GraphRep<K> {
    v: Vec<usize>,
    e: Vec<(usize, K, usize)>,
    pub id: Vec<(usize, usize, K)>,
}

impl<K: PartialOrd + Copy> GraphRep<K> {
    pub fn edges_from(&self, vertex: usize) -> &[(usize, K, usize)] {
        if vertex + 1 >= self.v.len() {
            panic!(
                "edges_from(): vertex {} out of range (v.len() = {})",
                vertex,
                self.v.len()
            );
        }
        let edges_start = self.v[vertex];
        let edges_end = self.v[vertex + 1];

        &self.e[edges_start..edges_end]
    }

    pub fn original_edge(&self, edge_id: usize) -> Option<&(usize, usize, K)> {
        self.id.get(edge_id)
    }

    pub fn num_vertices(&self) -> usize {
        self.v.len() - 1
    }

    pub fn num_edges(&self) -> usize {
        self.e.len() / 2
    }

    pub fn v_len(&self) -> usize {
        self.v.len()
    }

    pub fn e_len(&self) -> usize {
        self.e.len()
    }

    //function that can create a graph from a vec<(vertex, vertex, weight)>
    pub fn from_list(edges: Vec<(usize, usize, K)>) -> Self {
        let m = edges.len();
        if m == 0 {
            // zero vertices, zero edges, empty id
            return GraphRep {
                v: vec![0], // one offset, so edges_from(u) is never OOB
                e: Vec::new(),
                id: Vec::new(),
            };
        }
        let id = edges;
        let n = id
            .iter()
            .flat_map(|&(u, v, _)| [u, v])
            .max()
            .map_or(0, |mx| mx + 1);

        // number of edges at every vertex
        let mut v = vec![0; n + 1];
        for &(u, vtx, _) in &id {
            v[u] += 1;
            v[vtx] += 1;
        }

        // 3) fused prefix‐sum into `v` *and* init `write_cursor`
        let mut write_cursor = Vec::with_capacity(v.len());
        let mut running_sum = 0;
        for slot in &mut v {
            let deg = *slot; // old count
            write_cursor.push(running_sum);
            *slot = running_sum; // now CSR offset
            running_sum += deg;
        }

        // 4) allocate and scatter into `e`
        let dummy = (0, id[0].2, 0);
        let mut e = vec![dummy; 2 * m];
        for (edge_id, &(src, dst, weight)) in id.iter().enumerate() {
            // half‐edge src → dst
            let pos = write_cursor[src];
            e[pos] = (dst, weight, edge_id);
            write_cursor[src] += 1;

            // half‐edge dst → src
            let pos_back = write_cursor[dst];
            e[pos_back] = (src, weight, edge_id);
            write_cursor[dst] += 1;
        }

        GraphRep { v, e, id }
    }

    pub fn update_v_e(&mut self, edges: &[(usize, usize, K, usize)]) {
        if edges.is_empty() {
            self.v = vec![0]; // no vertices left
            self.e.clear(); // clear edge list
            return; // exit early
        }

        // Determine number of vertices (based on max index seen in edge list)
        let n = edges
            .iter()
            .flat_map(|&(u, v, _, _)| [u, v])
            .max()
            .unwrap_or(0)
            + 1;

        // Count the degree of each vertex to prepare CSR offsets
        let mut deg = vec![0; n + 1];
        for &(u, v, _, _) in edges {
            deg[u] += 1;
            deg[v] += 1;
        }

        // Compute prefix sums over degrees to build CSR offset vector `v`
        let mut v = vec![0; n + 1];
        for i in 1..=n {
            v[i] = v[i - 1] + deg[i - 1];
        }
        // Clone `v` as write cursor to track where to insert each neighbor
        let mut cursor = v.clone();

        // Allocate edge list of twice the input size (undirected = 2 half-edges)
        let mut e = vec![(0, edges[0].2, 0); 2 * edges.len()];

        // Scatter edges into CSR structure using cursor positions
        for &(u, vtx, w, id) in edges {
            e[cursor[u]] = (vtx, w, id);
            cursor[u] += 1;

            e[cursor[vtx]] = (u, w, id);
            cursor[vtx] += 1;
        }

        // Replace current CSR layout with newly built one
        self.v = v;
        self.e = e;
    }

    ///returns all original edges
    pub fn all_edges(&self) -> Vec<(usize, usize, K, usize)> {
        let mut result = Vec::new();

        for (eid, &(u, v, w)) in self.id.iter().enumerate() {
            if u < v {
                result.push((u, v, w, eid));
            } else {
                result.push((v, u, w, eid));
            }
        }
        result
    }

    pub fn current_edges(&self) -> Vec<(usize, usize, K, usize)> {
        let mut out = Vec::with_capacity(self.num_edges()); // m edges
        let n = self.num_vertices();

        for u in 0..n {
            for &(v, w, eid) in self.edges_from(u) {
                if u < v {
                    // keep one direction only
                    out.push((u, v, w, eid));
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::GraphRep;

    #[test]
    fn test_empty() {
        let g: GraphRep<i32> = GraphRep::from_list(Vec::new());
        assert_eq!(g.num_vertices(), 0);
        assert_eq!(g.num_edges(), 0);
        assert!(g.id.is_empty());
    }

    #[test]
    fn test_small_graph() {
        // edges = [(0–1,1), (1–2,2), (2–0,3)]
        let edges = vec![(0, 1, 1), (1, 2, 2), (2, 0, 3)];
        let g = GraphRep::from_list(edges.clone());

        assert_eq!(g.num_vertices(), 3);
        assert_eq!(g.num_edges(), 3);
        assert_eq!(g.id, edges);

        let degs: Vec<_> = (0..3).map(|u| g.edges_from(u).len()).collect();
        assert_eq!(degs, vec![2, 2, 2]);

        let mut adj0: Vec<_> = g.edges_from(0).iter().cloned().collect();
        adj0.sort_by_key(|&(to, _, eid)| (to, eid));
        assert_eq!(adj0, vec![(1, 1, 0), (2, 3, 2)]);
    }
}
