use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(long = "input")]
    input_file: String,
    #[arg(long = "output")]
    output_file: Option<String>,
}

#[derive(Debug)]
struct Graph {
    size: usize,
    coords: Vec<(i32, i32)>,
}

impl Graph {
    fn from_file(file_name: &str) -> Graph {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        let mut flag = false;
        let mut coords = vec![];
        let mut size = 0;
        for line in reader.lines() {
            let line = line.unwrap();

            if line.starts_with("DIMENSION: ") {
                let num: usize = line.split_whitespace().last().unwrap().parse().unwrap();
                size = num;
            } else if line == "EOF" {
                break;
            } else if line == "NODE_COORD_SECTION" {
                flag = true;
                println!("flag update");
            } else if flag {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                let (_id, x, y) = (nums[0], nums[1], nums[2]);
                coords.push((x, y));
            }
        }

        Graph { size, coords }
    }
}

#[derive(Debug)]
struct Solution<'a> {
    graph: &'a Graph,
    path: Vec<usize>,
}

impl<'a> Solution<'a> {
    fn new(graph: &'a Graph) -> Solution<'a> {
        let path = (0..graph.size).collect();
        Solution { graph, path }
    }

    fn from_file(graph: &'a Graph, file_name: &str) -> Solution<'a> {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        let mut path = vec![];
        let mut flag = false;
        let mut size = 0;
        for line in reader.lines() {
            let line = line.unwrap();

            if line.starts_with("DIMENSION: ") {
                let num: usize = line.split_whitespace().last().unwrap().parse().unwrap();
                size = num;
            } else if line == "-1" {
                break;
            } else if line == "TOUR_SECTION" {
                flag = true;
                println!("flag update");
            } else if flag {
                let id_node: usize = line.split_whitespace().last().unwrap().parse().unwrap();
                let id_node = id_node - 1;
                path.push(id_node);
            }
        }

        Solution { graph, path }
    }
}

fn main() {
    let cli = Cli::parse();
    println!("{}", cli.input_file);
    println!("{}", cli.output_file.as_ref().unwrap_or(&"No File".to_string()));

    let graph = Graph::from_file(&cli.input_file);
    let solution = if let Some(output_file) = cli.output_file {
        Solution::from_file(&graph, &output_file)
    } else {
        Solution::new(&graph)
    };

    // let solution = Solution::new(&graph);
    // println!("{:?}", graph);
    println!("{:?}", solution);
    // println!("{:#?}", solution);
}
