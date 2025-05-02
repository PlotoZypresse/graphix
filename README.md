# graphix

[![Crates.io](https://img.shields.io/crates/v/graphix.svg)](https://crates.io/crates/graphix)  [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight Rust library providing a compact **CSR** (Compressed-Sparse-Row) representation of **undirected graphs**, with full tracking of original edge IDs. Optimized for sparse graphs and supports weighted edges of any `Copy` type.

---

## Features

- **One-shot CSR construction** via `GraphRep::from_list(Vec<(u, v, weight)>)`
  infers vertex count and builds in **O(n + m)** time.
- **Fast adjacency access**:
  `edges_from(u) → &[(to, weight, edge_id)]`
- **Original‐edge lookup**:
  `original_edge(edge_id) → Option<&(u, v, weight)>`
- **Zero-panic on empty graphs** (handles 0-edge input gracefully).
- **Minimal private internals** (`v`, `e`) and a public `id` array of original edges.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
graphix = "0.4"
````

Or run:

```bash
cargo add graphix@0.4
```

---

## Quick Start

```rust
use graphix::GraphRep;

fn main() {
    // Suppose you read this Vec from your own I/O layer:
    let edges = vec![
        (0, 1, 2.5),  // undirected edge 0↔1 with weight 2.5
        (1, 2, 1.7),  // 1↔2 w = 1.7
        (2, 0, 3.1),  // 2↔0 w = 3.1
    ];

    // Build the CSR graph in one call:
    let graph: GraphRep<f64> = GraphRep::from_list(edges);

    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.num_edges(), 3);

    // Iterate adjacency of vertex 1:
    for &(to, weight, eid) in graph.edges_from(1) {
        println!("1 → {} (w = {}, edge_id = {})", to, weight, eid);
    }

    // Look up the original (u,v,weight) tuple for edge_id = 2
    if let Some(&(u, v, w)) = graph.original_edge(2) {
        println!("Original edge #2 was {}–{} (w = {})", u, v, w);
    }
}
```

---

## API

### `struct GraphRep<K>`

```rust
pub struct GraphRep<K> {
    v:  Vec<usize>,               // CSR offsets (private)
    e:  Vec<(usize, K, usize)>,   // (to, weight, edge_id) half-edges (private)
    pub id: Vec<(usize, usize, K)>, // your original edges: index = edge_id
}
```

#### Constructors

* `pub fn from_list(edges: Vec<(usize, usize, K)>) -> Self`
  Build a CSR graph from an edge-list.

  * Infers `n = max(u,v) + 1`
  * Handles `edges.is_empty()` → zero-vertex graph
  * Stores each undirected edge as two half-edges internally
  * Records your original `(u,v,weight)` tuples in `id`

#### Accessors

* `pub fn num_vertices(&self) -> usize`
  Returns the number of vertices `n`.

* `pub fn num_edges(&self) -> usize`
  Returns the number of undirected edges `m`.

* `pub fn edges_from(&self, u: usize) -> &[(usize, K, usize)]`
  Returns a slice of half-edges out of `u`:
  each tuple is `(to: usize, weight: K, edge_id: usize)`.

* `pub fn original_edge(&self, edge_id: usize) -> Option<&(usize, usize, K)>`
  Look up your original `(u, v, weight)` by `edge_id`.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
