# graphix

[![Crates.io](https://img.shields.io/crates/v/graphix.svg)](https://crates.io/crates/graphix)
[![docs.rs](https://docs.rs/graphix/badge.svg)](https://docs.rs/graphix)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight Rust library providing a compact **CSR** (Compressed Sparse Row) representation of **undirected graphs**, with full tracking of original edge IDs. Optimized for sparse graphs and supports weighted edges of any `Copy` type.

---

## Features

* **One-shot CSR construction** via `GraphRep::from_list(Vec<(u, v, weight)>)` in **O(n + m)**
* **Efficient adjacency access**:

  ```rust
  edges_from(u) → &[(to, weight, edge_id)]
  ```
* **Original-edge lookup**:

  ```rust
  original_edge(edge_id) → Option<&(u, v, weight)>
  ```
* **Dynamic CSR update** with existing edge IDs:

  ```rust
  update_v_e(&mut self, &[(u, v, weight, edge_id)])
  ```
* **Retrieve edge sets**:

  * `all_edges()` returns all original edges with IDs
  * `current_edges()` gives the current deduplicated edge list
* **Zero-panic on empty graphs**
* **Public `id` array for traceability**

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
graphix = "0.4.1"
```

Or use:

```bash
cargo add graphix@0.4.1
```

---

## Quick Start

```rust
use graphix::GraphRep;

fn main() {
    // an undirected triangle
    let edges = vec![
        (0, 1, 2.5),
        (1, 2, 1.7),
        (2, 0, 3.1),
    ];

    let mut graph = GraphRep::from_list(edges);

    // access neighbors of a vertex
    for (v, w, eid) in graph.edges_from(0) {
        println!("0 ↔ {} with weight {}, edge id {}", v, w, eid);
    }

    // extract all edges (original input)
    let all = graph.all_edges();

    // extract current edges from CSR (deduplicated)
    let current = graph.current_edges();

    // rebuild CSR from updated edge list
    graph.update_v_e(&current);
}
```

---

## API

### `struct GraphRep<K>`

```rust
pub struct GraphRep<K> {
    v: Vec<usize>,               // CSR offsets (private)
    e: Vec<(usize, K, usize)>,   // half-edges: (to, weight, edge_id) (private)
    pub id: Vec<(usize, usize, K)>, // original undirected edges
}
```

### Constructors

* `from_list(edges: Vec<(usize, usize, K)>) -> Self`
  Builds a graph in **O(n + m)**, infers `n = max(u,v)+1`, and stores all undirected edges with stable IDs.

### Accessors

* `num_vertices(&self) -> usize`
* `num_edges(&self) -> usize`
* `edges_from(&self, u: usize) -> &[(usize, K, usize)]`
* `original_edge(&self, edge_id: usize) -> Option<&(usize, usize, K)>`
* `all_edges(&self) -> Vec<(usize, usize, K, usize)>`
* `current_edges(&self) -> Vec<(usize, usize, K, usize)>`

### Mutators

* `update_v_e(&mut self, &[(usize, usize, K, usize)])`
  Rebuilds CSR structure from a fresh list of edges, retaining original edge IDs.

---

## License

This project is licensed under the [MIT License](LICENSE).
