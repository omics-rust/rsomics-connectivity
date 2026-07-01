//! Value-exact compat against networkx 3.6.1.
//!
//! λ(G) and κ(G) are graph invariants, so the expected values are exact
//! integers with no float tie-breaks. Goldens were produced once with
//! networkx 3.6.1 and are hardcoded / committed here; no Python or subprocess
//! runs at test time.

use rsomics_connectivity::{edge_connectivity_from_edge_list, node_connectivity_from_edge_list};

fn check(edges: &str, node: usize, edge: usize) {
    assert_eq!(
        node_connectivity_from_edge_list(edges),
        node,
        "node_connectivity mismatch"
    );
    assert_eq!(
        edge_connectivity_from_edge_list(edges),
        edge,
        "edge_connectivity mismatch"
    );
}

#[test]
fn path_graph() {
    check("0 1\n1 2\n2 3\n3 4\n", 1, 1);
}

#[test]
fn cycle_graph() {
    check("0 1\n1 2\n2 3\n3 4\n4 5\n5 0\n", 2, 2);
}

#[test]
fn complete_k5() {
    check("0 1\n0 2\n0 3\n0 4\n1 2\n1 3\n1 4\n2 3\n2 4\n3 4\n", 4, 4);
}

#[test]
fn complete_k7() {
    let mut e = String::new();
    for a in 0..7 {
        for b in (a + 1)..7 {
            e.push_str(&format!("{a} {b}\n"));
        }
    }
    check(&e, 6, 6);
}

#[test]
fn balanced_binary_tree() {
    // networkx.balanced_tree(2, 3): root 0, complete binary tree of depth 3.
    let e = "0 1\n0 2\n1 3\n1 4\n2 5\n2 6\n3 7\n3 8\n4 9\n4 10\n5 11\n5 12\n6 13\n6 14\n";
    check(e, 1, 1);
}

#[test]
fn two_triangles_joined_by_bridge() {
    let e = "a b\nb c\nc a\nd e\ne f\nf d\nc d\n";
    check(e, 1, 1);
}

#[test]
fn cut_vertex_two_edge_connected() {
    // Node κ=1 (removing c disconnects), edge λ=2 (the d–e–c cycle is 2-edge).
    let e = "a b\nb c\nc a\nc d\nd e\ne c\n";
    check(e, 1, 2);
}

#[test]
fn single_edge() {
    check("x y\n", 1, 1);
}

#[test]
fn disconnected_two_components() {
    check("a b\nc d\n", 0, 0);
}

#[test]
fn karate_club() {
    let e = include_str!("golden/karate.txt");
    check(e, 1, 1);
}

#[test]
fn gnm_sparse_induced() {
    // nx.gnm_random_graph(40, 60, seed=7); the edge-list-induced graph (38
    // nodes, isolated vertices absent) is connected with connectivity 1.
    let e = include_str!("golden/gnm_sparse.txt");
    check(e, 1, 1);
}

#[test]
fn gnm_medium() {
    // nx.gnm_random_graph(50, 150, seed=7)
    let e = include_str!("golden/gnm_medium.txt");
    check(e, 3, 3);
}

#[test]
fn gnm_dense() {
    // nx.gnm_random_graph(50, 400, seed=13)
    let e = include_str!("golden/gnm_dense.txt");
    check(e, 7, 7);
}
