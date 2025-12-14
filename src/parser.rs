use petgraph::graph::Graph;
pub mod expr;
pub mod token;

/// Parses the input string and returns a AST.
fn parse(input: &str) -> Graph<(), ()> {
    let mut graph: Graph<(), ()> = Graph::new();
    // Parsing logic would go here
    return graph;
}