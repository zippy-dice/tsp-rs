use clap::Parser;

mod graph;
mod solution;

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
}
