use clap::Parser;

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
    println!("{}", cli.input_file);
    println!("{}", cli.output_file.unwrap_or("No File".into()));
}
