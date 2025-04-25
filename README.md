
# graphix

[![Crates.io](https://img.shields.io/crates/v/graphix.svg)](https://crates.io/crates/graphix)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight Rust library providing a compact and efficient representation of **undirected graphs** using a compressed adjacency list format. Optimized for sparse graphs and supports weighted edges with customizable types.

---

## Features

- **GraphRep<K>**: a compressed adjacency list representation for undirected graphs.
- Efficient memory layout using flat vectors.
- Supports edge weights of any `Copy + Ord` type.
- Fast adjacency list access.
- Unit tested and ready for algorithmic use cases.

## Installation

Add `graphix` to your `Cargo.toml` dependencies:

```toml
[dependencies]
graphix = "0.1"
```

Or with `cargo`:

```bash
cargo add graphix
```

## Quick Start

```rust
use graphix::GraphRep;

fn main() {
    // Create a graph with 3 vertices and space for 2 edges
    let mut g: GraphRep<i32> = GraphRep::new(3, 2);

    // Add edges (undirected, so both directions are added)
    g.add_edge(0, 1, 10);
    g.add_edge(0, 2, 20);

    // Finalize internal layout for fast adjacency access
    g.finish_v();

    // Iterate over neighbors of vertex 0
    for (neighbor, weight) in g.edges_from(0) {
        println!("0 -> {} (weight {})", neighbor, weight);
    }
}
```

## API

### `GraphRep<K>`

- `GraphRep::new(n: usize, m: usize) -> Self`
  Create a graph with `n` vertices and space for `m` undirected edges.

- `add_edge(&mut self, u: usize, v: usize, weight: K)`
  Add an undirected edge between `u` and `v` with a given weight.

- `finish_v(&mut self)`
  Finalize the internal adjacency list structure. Must be called before using `edges_from`.

- `edges_from(&self, vertex: usize) -> Vec<(usize, K)>`
  Get a list of neighbor–weight pairs for a given vertex.

- `num_vertices(&self) -> usize`
  Return the number of vertices.

- `num_edges(&self) -> usize`
  Return the number of **undirected** edges.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
