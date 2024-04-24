use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Graph {
    pub size: usize,
    pub coords: Vec<(i32, i32)>,
}

impl Graph {
    pub fn from_file(file_name: &str) -> Graph {
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

    pub fn weight(&self, i: usize, j: usize) -> f32 {
        let (x1, y1) = self.coords[i];
        let (x2, y2) = self.coords[j];
        let norm = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
        (norm as f32).sqrt()
    }
}
