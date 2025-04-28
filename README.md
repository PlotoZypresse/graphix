# graphix

[![Crates.io](https://img.shields.io/crates/v/graphix.svg)](https://crates.io/crates/graphix)  [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight Rust library providing a compact and efficient representation of **undirected graphs** using a compressed adjacency list format. Optimized for sparse graphs and supports weighted edges with customizable types.

---

## Features

- **GraphRep<K>**: a compressed adjacency list representation for undirected graphs.
- Efficient memory layout using flat vectors.
- Supports edge weights of any `Copy + Ord` type.
- Fast adjacency list access.
- Construct directly from an edge list with `GraphRep::from_list`, inferring the number of vertices automatically.
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

### Manual construction

```rust
use graphix::GraphRep;

fn main() {
    // Create a graph with 3 vertices and space for 2 undirected edges
    let mut g: GraphRep<i32> = GraphRep::new(3, 2);

    // Add edges (undirected ⇒ both directions internally)
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

### From an edge list

```rust
use graphix::GraphRep;

fn main() {
    // Build directly from a Vec of (u, v, weight) triples:
    let edges = vec![
        (0, 1, 5),
        (1, 2, 7),
        (2, 0, 3),
    ];

    // No need to specify vertex count; it is inferred as max(u, v) + 1
    let g: GraphRep<i32> = GraphRep::from_list(edges);

    println!("Vertices: {}", g.num_vertices()); // 3
    println!("Edges: {}", g.num_edges());       // 3
}
```

## API

### `GraphRep<K>`

- `GraphRep::new(n: usize, m: usize) -> Self`
  Create a graph with `n` vertices and space for `m` undirected edges.

- `GraphRep::add_edge(&mut self, u: usize, v: usize, weight: K)`
  Add an undirected edge between `u` and `v` with a given weight. Internally stores each undirected edge as two directed entries.

- `GraphRep::finish_v(&mut self)`
  Finalize the internal adjacency list structure. Must be called before using `edges_from` when built manually.

- `GraphRep::edges_from(&self, vertex: usize) -> Vec<(usize, K)>`
  Get a list of (neighbor, weight) pairs for a given vertex.

- `GraphRep::num_vertices(&self) -> usize`
  Return the number of vertices.

- `GraphRep::num_edges(&self) -> usize`
  Return the number of **undirected** edges.

- `GraphRep::from_list(edges: Vec<(usize, usize, K)>) -> Self`
  Build a graph directly from an edge list. Infers the number of vertices as `max(u, v) + 1`, adds all edges, and finalizes the structure.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
