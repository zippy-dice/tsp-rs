use clap::Parser;

mod graph;
mod solution;
mod solver_sa;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(long = "input")]
    input_file: String,
    #[arg(long = "output")]
    output_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let graph = graph::Graph::from_file(&cli.input_file);
    let init_solution = solution::Solution::new(&graph);
    let solution = if let Some(output_file) = cli.output_file {
        solution::Solution::from_file(&graph, &output_file)
    } else {
        solution::Solution::new(&graph)
    };

    println!("init_solution: {}", init_solution.score());
    println!("input_solution: {}", solution.score());

    let graph = graph::Graph::from_file(&cli.input_file);
    let solver = solver_sa::SolverSA::from_graph(graph);
    let params = solver_sa::Params::new()
        .loops(1000000)
        .init_temperture(10.)
        .temperture_decay_rate(0.99999);
    let solution = solver.solve(&params);
    println!("optimized_solution: {}", solution.score());
}
