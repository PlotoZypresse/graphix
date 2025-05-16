# graphix

[![Crates.io](https://img.shields.io/crates/v/graphix.svg)](https://crates.io/crates/graphix)
[![docs.rs](https://docs.rs/graphix/badge.svg)](https://docs.rs/graphix)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight Rust library providing a compact **CSR** (Compressed-Sparse-Row) representation of **undirected graphs**, with full tracking of original edge IDs. Optimized for sparse graphs and supports weighted edges of any `Copy` type.

---

## Features

- **One-shot CSR construction** via `GraphRep::from_list(Vec<(u, v, weight)>)` in **O(n + m)**
- **Fast adjacency access**:
  ```rust
  edges_from(u) → &[(to, weight, edge_id)]
```

* **Original-edge lookup**:

  ```rust
  original_edge(edge_id) → Option<&(u, v, weight)>
  ```
* **In-place contraction** by connected-component IDs:

  ```rust
  contract_cc(&mut self, cc_ids: &[isize])
  ```
* **Zero-panic on empty graphs**
* **Minimal private internals** (`v`, `e`) and a public `id` array of original edges

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
graphix = "0.4.1"
```

Or run:

```bash
cargo add graphix@0.4.1
```

---

## Quick Start

```rust
use graphix::GraphRep;

fn main() {
    // an undirected triangle, with weights
    let edges = vec![
        (0, 1, 2.5),  // 0↔1 w=2.5
        (1, 2, 1.7),  // 1↔2 w=1.7
        (2, 0, 3.1),  // 2↔0 w=3.1
    ];

    let mut graph: GraphRep<f64> = GraphRep::from_list(edges);

    // contract two vertices together (e.g. merge 0 & 1)
    let cc_ids = vec![0isize, 0, 1];
    graph.contract_cc(&cc_ids);

    assert_eq!(graph.num_vertices(), 2);
    // graph.e now reflects edges between super-nodes 0 and 1 only
}
```

---

## API

### `struct GraphRep<K>`

```rust
pub struct GraphRep<K> {
    v:  Vec<usize>,               // CSR offsets (private)
    e:  Vec<(usize, K, usize)>,   // half-edges: (to, weight, edge_id) (private)
    pub id: Vec<(usize, usize, K)>, // your original undirected edges
}
```

#### Constructors

* `pub fn from_list(edges: Vec<(usize, usize, K)>) -> Self`
  Builds a CSR graph in **O(n + m)**, inferring `n = max(u,v)+1`, handling empty input, storing each undirected edge twice, and recording your original `(u,v,weight)` tuples in `id`.

#### Accessors

* `pub fn num_vertices(&self) -> usize` — number of vertices `n`.
* `pub fn num_edges(&self) -> usize` — number of undirected edges `m`.
* `pub fn edges_from(&self, u: usize) -> &[(usize, K, usize)]` — adjacency list for `u`.
* `pub fn original_edge(&self, edge_id: usize) -> Option<&(usize, usize, K)>` — lookup original edge.

#### Mutators

* `pub fn contract_cc(&mut self, cc_ids: &[isize])`
  In‐place contract this graph by assigning each vertex `u` the super‐node ID `cc_ids[u]`.

  * Rebuilds only `v` and `e` (the CSR fields), dropping self‐loops and carrying all original `edge_id`s through.
  * Leaves `id` untouched, so you can always trace back to your original edges.

---

## License

This project is licensed under the [MIT License](LICENSE).
