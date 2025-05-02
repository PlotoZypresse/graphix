/// representation for an undirected graph.
/// - v : vertex offsets, length n + 1.
///       Edges of vertex u are in e[v[u]] .. e[v[u + 1]].
/// - src : we need to remeber the origin vertex for each edge,
///         to order e after all edges are inserted
/// - e : indices into w, one entry per directed edge (each undirected
///       edge is stored twice, u→v and v→u).
/// - w : (neighbor, weight) tuples holding the actual edge data.
pub struct GraphRep<K> {
    v: Vec<usize>,
    //src: Vec<usize>,
    e: Vec<(usize, K, usize)>,
    //w: Vec<(usize, K)>,
    pub id: Vec<(usize, usize, K)>,
}

impl<K: PartialOrd + Copy> GraphRep<K> {
    pub fn edges_from(&self, vertex: usize) -> &[(usize, K, usize)] {
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
