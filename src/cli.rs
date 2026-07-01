use std::io::{self, Read};
use std::process::ExitCode;

use clap::{Parser, ValueEnum};
use rsomics_common::{run, CommonFlags, RsomicsError, ToolMeta};
use serde::Serialize;

use rsomics_connectivity::{edge_connectivity_from_edge_list, node_connectivity_from_edge_list};

pub const META: ToolMeta = ToolMeta {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
};

#[derive(Copy, Clone, PartialEq, Eq, Debug, ValueEnum)]
pub enum Mode {
    /// λ(G): minimum edges to remove to disconnect the graph.
    Edge,
    /// κ(G): minimum nodes to remove to disconnect the graph.
    Node,
}

impl Mode {
    fn as_str(self) -> &'static str {
        match self {
            Mode::Edge => "edge",
            Mode::Node => "node",
        }
    }
}

/// Exact node/edge connectivity of an undirected graph
/// (`networkx.node_connectivity` / `networkx.edge_connectivity`).
///
/// Reads an undirected edge list from stdin (`u v` per line, string labels;
/// `#` comments and blank lines skipped; parallel edges deduplicated;
/// self-loops dropped — as in `nx.read_edgelist` → `nx.Graph`). An edge list
/// cannot express isolated nodes, so connectivity is computed over the graph
/// induced by the edges. Output is the single integer connectivity value.
#[derive(Parser, Debug)]
#[command(name = "rsomics-connectivity", version, about, long_about = None)]
pub struct Cli {
    /// Which connectivity to compute.
    #[arg(long, value_enum, default_value_t = Mode::Edge)]
    pub mode: Mode,

    #[command(flatten)]
    pub common: CommonFlags,
}

#[derive(Serialize)]
struct Output {
    connectivity: usize,
    mode: &'static str,
}

impl Cli {
    pub fn run(self) -> ExitCode {
        let common = self.common.clone();
        run(&common, META, || {
            let mut input = String::new();
            io::stdin()
                .lock()
                .read_to_string(&mut input)
                .map_err(RsomicsError::Io)?;

            let connectivity = match self.mode {
                Mode::Edge => edge_connectivity_from_edge_list(&input),
                Mode::Node => node_connectivity_from_edge_list(&input),
            };

            if !common.json {
                println!("{connectivity}");
            }
            Ok(Output {
                connectivity,
                mode: self.mode.as_str(),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn cli_debug_assert() {
        super::Cli::command().debug_assert();
    }
}
