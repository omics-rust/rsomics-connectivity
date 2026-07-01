# rsomics-connectivity

Exact **node** and **edge** connectivity of an undirected graph — a value-exact
Rust port of `networkx.node_connectivity` and `networkx.edge_connectivity`.

- `--mode edge` (default) → **λ(G)**: the minimum number of edges whose removal
  disconnects the graph (0 if already disconnected).
- `--mode node` → **κ(G)**: the minimum number of nodes whose removal
  disconnects the graph or renders it trivial (n−1 for the complete graph
  K_n; 0 if disconnected).

Both are well-defined integer graph invariants, so the output is the same exact
integer NetworkX returns, computed here via integer-indexed Dinic max-flow.

## Install

```sh
cargo install rsomics-connectivity
```

## Usage

Reads an undirected edge list from stdin — one `u v` per line, arbitrary string
node labels, `#` comments and blank lines skipped, parallel edges deduplicated,
self-loops dropped (matching `networkx.read_edgelist` → `nx.Graph`).

```sh
# edge connectivity (default)
printf '0 1\n1 2\n2 3\n3 0\n' | rsomics-connectivity
# 2

# node connectivity
printf '0 1\n1 2\n2 3\n3 0\n' | rsomics-connectivity --mode node
# 2

# JSON envelope
printf '0 1\n1 2\n' | rsomics-connectivity --mode edge --json
# {"schema_version":"1.0","tool":"rsomics-connectivity",...,"result":{"connectivity":1,"mode":"edge"}}
```

An edge list cannot express isolated (degree-0) nodes, so connectivity is
computed over the graph **induced by the edges** — exactly as `nx.read_edgelist`
does. To model isolated nodes, add them as their own component upstream.

## Origin

This crate is an independent Rust reimplementation of NetworkX's flow-based
connectivity routines, based on:

- NetworkX 3.6.1 (`networkx.algorithms.connectivity.connectivity`,
  BSD-3-Clause), whose source was read and cited.
- S. Even, *An Algorithm for Determining Whether the Connectivity of a Graph is
  at Least k*, SIAM J. Comput. 4(3), 1975 — the min-degree-vertex node
  connectivity optimization.
- A.-H. Esfahanian, *Connectivity Algorithms* — the reference underlying
  NetworkX's implementation.
- E. A. Dinic, *Algorithm for solution of a problem of maximum flow in a network
  with power estimation*, 1970 — the max-flow engine used here.

Because λ(G) and κ(G) are graph invariants, any correct exact max-flow/min-cut
algorithm yields the same integers; this port does not replicate NetworkX's
Edmonds-Karp internals, only its exact results, verified against NetworkX 3.6.1
on hand graphs, the karate club, and `gnm` random graphs.

License: MIT OR Apache-2.0.
Upstream credit: NetworkX <https://networkx.org/> (BSD-3-Clause).
