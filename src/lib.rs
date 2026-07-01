//! Exact node/edge connectivity of an undirected graph.
//!
//! λ(G) and κ(G) are graph invariants: any correct exact max-flow/min-cut
//! algorithm yields the same integers as networkx's flow-based routines. We use
//! Dinic max-flow on integer-indexed graphs.

mod dinic;
mod graph;

use dinic::Dinic;
use graph::Graph;

/// Edge connectivity λ(G): minimum number of edges whose removal disconnects
/// the graph (0 if already disconnected, 0 for ≤1 node).
///
/// Undirected unit-capacity max-flow: λ = min over t≠s of maxflow(s→t) for a
/// fixed source s. Each undirected edge becomes two directed arcs of capacity 1.
#[must_use]
pub fn edge_connectivity(g: &Graph) -> usize {
    let n = g.node_count();
    if n <= 1 {
        return 0;
    }

    let mut dinic = Dinic::new(n);
    for u in 0..n {
        for &v in g.neighbors(u) {
            if u < v {
                dinic.add_arc(u, v, 1);
                dinic.add_arc(v, u, 1);
            }
        }
    }

    let s = 0;
    let mut lambda = usize::MAX;
    for t in 1..n {
        dinic.reset();
        lambda = lambda.min(dinic.max_flow(s, t, lambda));
        if lambda == 0 {
            return 0;
        }
    }
    lambda
}

/// Node connectivity κ(G): minimum number of nodes whose removal disconnects
/// the graph or renders it trivial (n−1 for the complete graph K_n, 0 if
/// disconnected, 0 for ≤1 node).
///
/// Node-split max-flow: each node v → v_in --cap1--> v_out; each original edge
/// u–v → u_out --∞--> v_in and v_out --∞--> u_in. κ = min over non-adjacent
/// pairs of local node connectivity, using the min-degree-vertex optimization
/// (Even 1975; networkx algorithm 11 / Esfahanian).
#[must_use]
pub fn node_connectivity(g: &Graph) -> usize {
    let n = g.node_count();
    if n <= 1 {
        return 0;
    }
    if !g.is_connected() {
        return 0;
    }

    // Complete graph: no non-adjacent pair exists; κ = n − 1.
    let edges = g.edge_count();
    if edges == n * (n - 1) / 2 {
        return n - 1;
    }

    // Split node v into v_in = 2v and v_out = 2v + 1.
    let cap_inf = n; // any flow ≥ n saturates; unit internal caps bound κ ≤ n−1
    let mut dinic = Dinic::new(2 * n);
    for v in 0..n {
        dinic.add_arc(2 * v, 2 * v + 1, 1);
    }
    for u in 0..n {
        for &v in g.neighbors(u) {
            if u < v {
                dinic.add_arc(2 * u + 1, 2 * v, cap_inf);
                dinic.add_arc(2 * v + 1, 2 * u, cap_inf);
            }
        }
    }

    // Pick a minimum-degree vertex v; κ ≤ deg(v) is the starting bound.
    let mut v = 0;
    let mut kappa = g.degree(0);
    for u in 1..n {
        let d = g.degree(u);
        if d < kappa {
            kappa = d;
            v = u;
        }
    }

    let flow = |dinic: &mut Dinic, a: usize, b: usize, cutoff: usize| {
        dinic.reset();
        dinic.max_flow(2 * a + 1, 2 * b, cutoff)
    };

    // Local node connectivity between v and each of its non-neighbours.
    for w in 0..n {
        if w == v || g.is_adjacent(v, w) {
            continue;
        }
        kappa = kappa.min(flow(&mut dinic, v, w, kappa));
    }

    // Non-adjacent pairs among v's neighbours.
    let neigh = g.neighbors(v);
    for i in 0..neigh.len() {
        for j in (i + 1)..neigh.len() {
            let (x, y) = (neigh[i], neigh[j]);
            if g.is_adjacent(x, y) {
                continue;
            }
            kappa = kappa.min(flow(&mut dinic, x, y, kappa));
        }
    }

    kappa
}

/// Parse an undirected edge list and return λ(G).
#[must_use]
pub fn edge_connectivity_from_edge_list(input: &str) -> usize {
    edge_connectivity(&Graph::from_edge_list(input))
}

/// Parse an undirected edge list and return κ(G).
#[must_use]
pub fn node_connectivity_from_edge_list(input: &str) -> usize {
    node_connectivity(&Graph::from_edge_list(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_is_one_one() {
        let e = "0 1\n1 2\n2 3\n3 4\n";
        assert_eq!(edge_connectivity_from_edge_list(e), 1);
        assert_eq!(node_connectivity_from_edge_list(e), 1);
    }

    #[test]
    fn cycle_is_two_two() {
        let e = "0 1\n1 2\n2 3\n3 4\n4 5\n5 0\n";
        assert_eq!(edge_connectivity_from_edge_list(e), 2);
        assert_eq!(node_connectivity_from_edge_list(e), 2);
    }

    #[test]
    fn k5_is_four_four() {
        let mut e = String::new();
        for a in 0..5 {
            for b in (a + 1)..5 {
                e.push_str(&format!("{a} {b}\n"));
            }
        }
        assert_eq!(edge_connectivity_from_edge_list(&e), 4);
        assert_eq!(node_connectivity_from_edge_list(&e), 4);
    }

    #[test]
    fn disconnected_is_zero() {
        let e = "a b\nc d\n";
        assert_eq!(edge_connectivity_from_edge_list(e), 0);
        assert_eq!(node_connectivity_from_edge_list(e), 0);
    }

    #[test]
    fn single_edge_is_one() {
        let e = "x y\n";
        assert_eq!(edge_connectivity_from_edge_list(e), 1);
        assert_eq!(node_connectivity_from_edge_list(e), 1);
    }
}
