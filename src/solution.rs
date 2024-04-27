use rand;
use rand::Rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    pub fn random_2_opt(&mut self) {
        let mut rng = rand::thread_rng();
        let size = self.graph.size;
        let mut idx1 = rng.gen_range(0..size);
        let mut idx2 = rng.gen_range(0..size);
        while idx1 == idx2 {
            idx2 = rng.gen_range(0..size); // 同じインデックスが選ばれないようにする
        }

        if idx1 > idx2 {
            (idx1, idx2) = (idx2, idx1);
        }

        self.swap_2_opt(idx1, idx2);
    }

    pub fn swap_2_opt(&mut self, i: usize, j: usize) {
        let (mut i, mut j) = (i, j);
        if i > j {
            (i, j) = (j, i);
        }
        self.path[i..=j].reverse();
    }

    pub fn random_3_opt(&mut self) {
        let mut rng = rand::thread_rng();
        let size = self.graph.size;

        let (idx_a, idx_b, idx_c) = {
            let mut nums = HashSet::new();
            while nums.len() < 3 {
                nums.insert(rng.gen_range(0..size));
            }
            let mut vec: Vec<_> = nums.into_iter().collect::<Vec<_>>();
            vec.sort();
            (vec[0], vec[1], vec[2])
        };

        let mut swap = rng.gen_bool(0.5);
        let mut rev1 = rng.gen_bool(0.5);
        let mut rev2 = rng.gen_bool(0.5);
        while !swap && !rev1 && !rev2 {
            swap = rng.gen_bool(0.5);
            rev1 = rng.gen_bool(0.5);
            rev2 = rng.gen_bool(0.5);
        }

        self.swap_3_opt(idx_a, idx_b, idx_c, swap, rev1, rev2);
    }

    pub fn swap_3_opt(&mut self, idx_a: usize, idx_b: usize, idx_c: usize, swap: bool, rev1: bool, rev2: bool) {
        assert!(idx_a <= idx_b && idx_b <= idx_c);
        let size = self.graph.size;

        let mut segments: Vec<Vec<usize>> = Vec::new();
        segments.push(self.path[..=idx_a].into());
        segments.push(self.path[idx_a + 1..=idx_b].into());
        segments.push(self.path[idx_b + 1..=idx_c].into());
        segments.push(self.path[idx_c + 1..size].into());

        if swap {
            segments[1..=2].reverse();
        }

        if rev1 {
            segments[1].reverse();
        }

        if rev2 {
            segments[2].reverse();
        }
        let mut path: Vec<usize> = Vec::new();
        for i in 0..4 {
            path.extend(segments[i].iter());
        }
        self.path = path;
    }

    pub fn score(&self) -> f32 {
        let score: f32 = self
            .path
            .windows(2)
            .map(|ws| (ws[0], ws[1]))
            .map(|(i, j)| self.graph.weight(i, j))
            .sum::<f32>()
            + self
                .graph
                .weight(self.path[self.path.len() - 1], self.path[0]);

        score
    }
}
