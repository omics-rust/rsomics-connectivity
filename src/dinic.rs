//! Dinic max-flow on integer-indexed nodes with a flat edge array and reusable
//! BFS/DFS buffers. Arcs are stored in reverse-paired order so arc `i` pairs
//! with arc `i ^ 1`. `reset()` restores original capacities so one instance
//! serves the many s→t flow computations connectivity requires.

pub struct Dinic {
    head: Vec<Vec<usize>>,
    to: Vec<usize>,
    cap: Vec<usize>,
    orig_cap: Vec<usize>,
    level: Vec<i32>,
    iter: Vec<usize>,
    queue: Vec<usize>,
}

impl Dinic {
    pub fn new(n: usize) -> Self {
        Self {
            head: vec![Vec::new(); n],
            to: Vec::new(),
            cap: Vec::new(),
            orig_cap: Vec::new(),
            level: vec![-1; n],
            iter: vec![0; n],
            queue: Vec::with_capacity(n),
        }
    }

    /// Add a directed arc `u → v` of the given capacity plus its residual pair.
    pub fn add_arc(&mut self, u: usize, v: usize, cap: usize) {
        let a = self.to.len();
        self.head[u].push(a);
        self.to.push(v);
        self.cap.push(cap);
        let b = self.to.len();
        self.head[v].push(b);
        self.to.push(u);
        self.cap.push(0);
        self.orig_cap.push(cap);
        self.orig_cap.push(0);
    }

    pub fn reset(&mut self) {
        self.cap.copy_from_slice(&self.orig_cap);
    }

    fn bfs(&mut self, s: usize, t: usize) -> bool {
        self.level.iter_mut().for_each(|l| *l = -1);
        self.queue.clear();
        self.level[s] = 0;
        self.queue.push(s);
        let mut qi = 0;
        while qi < self.queue.len() {
            let u = self.queue[qi];
            qi += 1;
            for &e in &self.head[u] {
                let v = self.to[e];
                if self.cap[e] > 0 && self.level[v] < 0 {
                    self.level[v] = self.level[u] + 1;
                    self.queue.push(v);
                }
            }
        }
        self.level[t] >= 0
    }

    fn dfs(&mut self, u: usize, t: usize, pushed: usize) -> usize {
        if u == t {
            return pushed;
        }
        while self.iter[u] < self.head[u].len() {
            let e = self.head[u][self.iter[u]];
            let v = self.to[e];
            if self.cap[e] > 0 && self.level[v] == self.level[u] + 1 {
                let d = self.dfs(v, t, pushed.min(self.cap[e]));
                if d > 0 {
                    self.cap[e] -= d;
                    self.cap[e ^ 1] += d;
                    return d;
                }
            }
            self.iter[u] += 1;
        }
        0
    }

    /// Maximum flow from `s` to `t`, terminating early once flow reaches
    /// `cutoff` (connectivity only needs the running minimum, so no point
    /// pushing past the current bound).
    pub fn max_flow(&mut self, s: usize, t: usize, cutoff: usize) -> usize {
        let mut flow = 0;
        while flow < cutoff && self.bfs(s, t) {
            self.iter.iter_mut().for_each(|i| *i = 0);
            loop {
                let f = self.dfs(s, t, usize::MAX);
                if f == 0 {
                    break;
                }
                flow += f;
                if flow >= cutoff {
                    break;
                }
            }
        }
        flow
    }
}
