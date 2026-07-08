use std::collections::HashMap;

/// Undirected simple graph over interned integer node IDs (`0..n`), matching
/// `networkx.read_edgelist` → `nx.Graph`: `#` comments and blank lines skipped,
/// parallel edges deduplicated, self-loops dropped. The edge list cannot
/// express isolated nodes, so the graph is the one induced by the edges.
pub struct Graph {
    adj: Vec<Vec<usize>>,
    edge_count: usize,
}

impl Graph {
    pub fn from_edge_list(input: &str) -> Self {
        let mut ids: HashMap<String, usize> = HashMap::new();
        let mut adj: Vec<Vec<usize>> = Vec::new();

        for line in input.lines() {
            // nx.parse_edgelist strips a '#' comment anywhere in the line before tokenising.
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            let mut it = line.split_whitespace();
            let (Some(a), Some(b)) = (it.next(), it.next()) else {
                continue;
            };
            let u = intern(a, &mut ids, &mut adj);
            let v = intern(b, &mut ids, &mut adj);
            if u != v && !adj[u].contains(&v) {
                adj[u].push(v);
                adj[v].push(u);
            }
        }

        let edge_count = adj.iter().map(Vec::len).sum::<usize>() / 2;
        Self { adj, edge_count }
    }

    pub fn node_count(&self) -> usize {
        self.adj.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn neighbors(&self, v: usize) -> &[usize] {
        &self.adj[v]
    }

    pub fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    pub fn is_adjacent(&self, u: usize, v: usize) -> bool {
        self.adj[u].contains(&v)
    }

    pub fn is_connected(&self) -> bool {
        let n = self.node_count();
        if n == 0 {
            return false;
        }
        let mut seen = vec![false; n];
        let mut stack = vec![0usize];
        seen[0] = true;
        let mut count = 1;
        while let Some(u) = stack.pop() {
            for &w in &self.adj[u] {
                if !seen[w] {
                    seen[w] = true;
                    count += 1;
                    stack.push(w);
                }
            }
        }
        count == n
    }
}

fn intern(name: &str, ids: &mut HashMap<String, usize>, adj: &mut Vec<Vec<usize>>) -> usize {
    if let Some(&id) = ids.get(name) {
        return id;
    }
    let id = adj.len();
    adj.push(Vec::new());
    ids.insert(name.to_owned(), id);
    id
}
