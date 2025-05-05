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

    pub fn update_v_e(&mut self, edges: &[(usize, usize, K, usize)]) {
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

    pub fn contract_cc(&mut self, cc_ids: &[isize]) {
        // old number of vertices and number of supernodes
        let org_vert_count = self.num_vertices();
        debug_assert_eq!(cc_ids.len(), org_vert_count);

        let supernode_count = cc_ids
            .iter()
            .map(|&label| label as usize)
            .max()
            .unwrap_or(0)
            + 1;

        //create a new offsett array (v), and couint half edges
        let mut new_offsets = vec![0usize; supernode_count + 1];

        for old_vert_index in 0..org_vert_count {
            let super_node_id = cc_ids[old_vert_index] as usize;
            for &(neighbor_old_index, _edge_weight, _org_edge_id) in self.edges_from(old_vert_index)
            {
                let neighbor_super_id = cc_ids[neighbor_old_index] as usize;

                if super_node_id != neighbor_super_id {
                    new_offsets[super_node_id + 1] += 1;
                }
            }
        }

        for i in 1..=supernode_count {
            new_offsets[i] += new_offsets[i - 1];
        }

        let total_edges = new_offsets[supernode_count];
        let dummy_edges = (0, self.e[0].1, 0);
        let mut new_edges = vec![dummy_edges; total_edges];

        let mut write_cursor = new_offsets.clone();

        for old_vert_index in 0..org_vert_count {
            let super_node_id = cc_ids[old_vert_index] as usize;
            for &(neighbor_old_index, edge_weight, org_edge_id) in self.edges_from(old_vert_index) {
                let neighbor_super_id = cc_ids[neighbor_old_index] as usize;
                if super_node_id != neighbor_super_id {
                    let write_pos = write_cursor[super_node_id];
                    new_edges[write_pos] = (neighbor_super_id, edge_weight, org_edge_id);
                    write_cursor[super_node_id] += 1;
                }
            }
        }
        self.v = new_offsets;
        self.e = new_edges
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

#[cfg(test)]
mod contract_tests {
    use super::GraphRep;

    #[test]
    fn test_contract_cc_chain() {
        // Original graph: a path 0–1–2–3 with weights 10, 20, 30
        let edges = vec![(0, 1, 10), (1, 2, 20), (2, 3, 30)];
        let mut g: GraphRep<i32> = GraphRep::from_list(edges.clone());

        // Verify original graph structure
        assert_eq!(g.num_vertices(), 4);
        assert_eq!(g.num_edges(), 3);

        // Define CC IDs that merge {0,1} into super-node 0 and {2,3} into super-node 1
        let cc_ids = vec![0isize, 0, 1, 1];

        // Contract in-place
        g.contract_cc(&cc_ids);

        // After contraction we expect:
        // - 2 super-nodes (0 and 1)
        // - 1 undirected edge between them (the former 1–2 edge)
        assert_eq!(g.num_vertices(), 2);
        assert_eq!(g.num_edges(), 1);

        // CSR offsets should be [0, 1, 2]:
        //  node 0 has 1 half-edge, node 1 has 1 half-edge
        assert_eq!(g.v, vec![0, 1, 2]);

        // The only remaining edge should have:
        //  - from super-node 0 → 1, weight 20, orig_eid = 1
        let out0 = g.edges_from(0);
        assert_eq!(out0.len(), 1);
        assert_eq!(out0[0], (1, 20, 1));

        // And the symmetric half-edge from 1 → 0
        let out1 = g.edges_from(1);
        assert_eq!(out1.len(), 1);
        assert_eq!(out1[0], (0, 20, 1));
    }

    #[test]
    fn test_contract_cc_identity() {
        // If cc_ids is [0,1,2,3], nothing should change
        let edges = vec![(0, 1, 5), (1, 2, 6), (2, 3, 7)];
        let mut g1: GraphRep<i32> = GraphRep::from_list(edges.clone());
        let original_v = g1.v.clone();
        let original_e = g1.e.clone();

        let cc_ids = vec![0isize, 1, 2, 3];
        g1.contract_cc(&cc_ids);

        // Vertex and edge counts unchanged
        assert_eq!(g1.num_vertices(), 4);
        assert_eq!(g1.num_edges(), 3);
        // CSR arrays identical
        assert_eq!(g1.v, original_v);
        assert_eq!(g1.e, original_e);
    }
}
