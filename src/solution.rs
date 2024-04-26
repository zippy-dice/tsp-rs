use std::fs::File;
use std::io::{BufRead, BufReader};

use rand;
use rand::Rng;

use crate::graph::Graph;

#[derive(Debug, Clone)]
pub struct Solution<'a> {
    graph: &'a Graph,
    path: Vec<usize>,
}

impl<'a> Solution<'a> {
    pub fn new(graph: &'a Graph) -> Solution<'a> {
        let path = (0..graph.size).collect();
        Solution { graph, path }
    }

    pub fn from_file(graph: &'a Graph, file_name: &str) -> Solution<'a> {
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
            } else if flag {
                let id_node: usize = line.split_whitespace().last().unwrap().parse().unwrap();
                let id_node = id_node - 1;
                path.push(id_node);
            }
        }

        Solution { graph, path }
    }

    pub fn swap_2_opt(&mut self) {
        let mut rng = rand::thread_rng();
        let size = self.graph.size;
        let idx1 = rng.gen_range(0..size);
        let mut idx2 = rng.gen_range(0..size);
        while idx1 == idx2 {
            idx2 = rng.gen_range(0..size); // 同じインデックスが選ばれないようにする
        }

        // TODO リバースする

        self.path.swap(idx1, idx2)
    }

    pub fn score(&self) -> f32 {
        let score = self
            .path
            .windows(2)
            .map(|ws| (ws[0], ws[1]))
            .map(|(i, j)| self.graph.weight(i, j))
            .sum();

        score
    }
}
