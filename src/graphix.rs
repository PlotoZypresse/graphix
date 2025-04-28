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
    src: Vec<usize>,
    e: Vec<usize>,
    w: Vec<(usize, K)>,
}

impl<K: PartialOrd + Copy> GraphRep<K> {
    //creates an empty graph that holds n verticies and m edges
    // e and w have size m*2 as each edge is added twice one for u to v and one for v to u
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            v: vec![0; n + 1],
            src: Vec::with_capacity(m * 2),
            e: Vec::with_capacity(m * 2),
            w: Vec::with_capacity(m * 2),
        }
    }

    // Adds the edge from u to v and its weight
    // Also adds the edge v to u and its weight as its an undirected graph
    pub fn add_edge(&mut self, u: usize, v: usize, weight: K) {
        // TODO: push into self.e and self.w, increment counts in self.v

        //add the edges endpoint and weight to w
        let index_uv = self.w.len();
        self.w.push((v, weight));
        self.src.push(u);
        self.e.push(index_uv);
        self.v[u] += 1;

        let index_vu = self.w.len();
        self.w.push((u, weight));
        self.src.push(v);
        self.e.push(index_vu);
        self.v[v] += 1;
    }

    pub fn edges_from(&self, vertex: usize) -> Vec<(usize, K)> {
        let edges_start = self.v[vertex];
        let edges_end = self.v[vertex + 1];

        self.e[edges_start..edges_end]
            .iter()
            .map(|&index_w| self.w[index_w])
            .collect()
    }

    // converts the edge count that is stored for each vertex
    // to the start index of the edges of that vertex in e
    // transforms v into a prefix sum vec!
    // called once after all edges are added to v
    pub fn finish_v(&mut self) {
        let mut index_sum: usize = 0;

        for index in 0..self.v.len() {
            let count = self.v[index];
            self.v[index] = index_sum;
            index_sum += count;
        }

        // group edges after their origin vertex using vcec! src
        // necessary as self e is in insertion order
        // Assumption that the input is not orderd after vertices
        let mut new_e = vec![0; self.e.len()];

        let mut next_slot = self.v.clone();

        for entry_indx in 0..self.e.len() {
            let index_w = self.e[entry_indx];

            //get the origin vertex for u
            let u = self.src[entry_indx];

            // get the new destination in 'e' from the cloned 'v'
            let destination = next_slot[u];

            // place the edge in the correct slot
            new_e[destination] = index_w;

            // increment to the next free slot
            next_slot[u] += 1;
        }

        self.e = new_e;
    }

    pub fn num_vertices(&self) -> usize {
        self.v.len() - 1
    }

    pub fn num_edges(&self) -> usize {
        self.w.len() / 2
    }

    //function that can create a graph from a vec<(vertex, vertex, weight)>
    pub fn from_list(edges: Vec<(usize, usize, K)>) -> Self {
        if edges.is_empty() {
            return GraphRep::new(0, 0);
        }

        let max_vertex = edges.iter().flat_map(|&(u, v, _)| [u, v]).max().unwrap();
        let n = max_vertex + 1;
        let m = edges.len();

        let mut g = GraphRep::new(n, m);
        for (u, v, w) in edges {
            g.add_edge(u, v, w);
        }
        g.finish_v();
        g
    }
}

#[cfg(test)]
mod tests {
    use super::GraphRep;

    #[test]
    fn test_new() {
        let g: GraphRep<i32> = GraphRep::new(5, 0);
        // num_vertices should be n
        assert_eq!(g.num_vertices(), 5);
        // no edges yet
        assert_eq!(g.num_edges(), 0);
        // edges_from on an empty graph (pre‐finish_v) returns empty Vec
        for u in 0..5 {
            assert!(g.edges_from(u).is_empty());
        }
    }

    #[test]
    fn test_add_edge_and_num_edges() {
        let mut g: GraphRep<i32> = GraphRep::new(2, 1);
        assert_eq!(g.num_edges(), 0);
        g.add_edge(0, 1, 10);
        // now exactly one undirected edge
        assert_eq!(g.num_edges(), 1);
        // vertex count unchanged
        assert_eq!(g.num_vertices(), 2);
    }

    #[test]
    fn test_num_edges_multiple() {
        let mut g: GraphRep<i32> = GraphRep::new(4, 3);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 2);
        g.add_edge(2, 3, 3);
        assert_eq!(g.num_edges(), 3);
    }

    #[test]
    fn test_finish_v_and_edges_from() {
        let mut g: GraphRep<i32> = GraphRep::new(3, 2);
        // add two edges from vertex 0
        g.add_edge(0, 1, 100);
        g.add_edge(0, 2, 200);

        // now rearrange v into start‐offsets and order e
        g.finish_v();

        // collect and sort to compare
        let mut edges0 = g.edges_from(0);
        edges0.sort_by_key(|&(nbr, _)| nbr);
        assert_eq!(edges0, vec![(1, 100), (2, 200)]);

        // the reverse half‐edges
        assert_eq!(g.edges_from(1), vec![(0, 100)]);
        assert_eq!(g.edges_from(2), vec![(0, 200)]);
    }

    #[test]
    fn test_edges_from_empty_after_finish() {
        let mut g: GraphRep<i32> = GraphRep::new(3, 0);
        g.finish_v();
        // still no edges anywhere
        for u in 0..3 {
            assert!(g.edges_from(u).is_empty());
        }
    }

    #[test]
    fn test_full_process() {
        let mut g: GraphRep<usize> = GraphRep::new(5, 10);
        // build a small undirected graph:
        //   0—1
        //   │
        //   2
        // and 3—4
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 2);
        g.add_edge(1, 2, 3);
        g.add_edge(3, 4, 4);

        // we’ve added 4 undirected edges
        assert_eq!(g.num_edges(), 4);

        // finalize v and reorder e
        g.finish_v();

        // check vertex degrees
        let degrees: Vec<usize> = (0..5).map(|u| g.edges_from(u).len()).collect();
        assert_eq!(degrees, vec![2, 2, 2, 1, 1]);

        // and some specific adjacency‐list checks
        let mut adj0 = g.edges_from(0);
        adj0.sort_by_key(|&(nbr, _)| nbr);
        assert_eq!(adj0, vec![(1, 1), (2, 2)]);

        assert_eq!(g.edges_from(3), vec![(4, 4)]);
        assert_eq!(g.edges_from(4), vec![(3, 4)]);
    }

    #[test]
    fn test_from_edge_list_empty() {
        // empty input → empty graph
        let g: GraphRep<i32> = GraphRep::from_list(Vec::new());
        assert_eq!(g.num_vertices(), 0);
        assert_eq!(g.num_edges(), 0);
    }

    #[test]
    fn test_from_edge_list_triangle() {
        // a 3-cycle: 0–1 (w=1), 1–2 (w=2), 2–0 (w=3)
        let edges = vec![(0, 1, 1), (1, 2, 2), (2, 0, 3)];
        let g = GraphRep::from_list(edges);

        // we saw vertices 0,1,2 → n = 3
        assert_eq!(g.num_vertices(), 3);
        // three undirected edges
        assert_eq!(g.num_edges(), 3);

        // each vertex in a 3-cycle has degree 2
        let degrees: Vec<usize> = (0..3).map(|u| g.edges_from(u).len()).collect();
        assert_eq!(degrees, vec![2, 2, 2]);

        // check that the weights match
        let mut adj0 = g.edges_from(0);
        adj0.sort_by_key(|&(nbr, _)| nbr);
        assert_eq!(adj0, vec![(1, 1), (2, 3)]);

        let mut adj1 = g.edges_from(1);
        adj1.sort_by_key(|&(nbr, _)| nbr);
        assert_eq!(adj1, vec![(0, 1), (2, 2)]);

        let mut adj2 = g.edges_from(2);
        adj2.sort_by_key(|&(nbr, _)| nbr);
        assert_eq!(adj2, vec![(0, 3), (1, 2)]);
    }
}
