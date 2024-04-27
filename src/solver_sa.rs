use crate::graph::Graph;
use crate::solution::Solution;

use rand;
use rand::Rng;

pub struct SolverSA {
    graph: Graph,
}

impl SolverSA {
    pub fn from_graph(graph: Graph) -> SolverSA {
        SolverSA { graph }
    }

    pub fn solve(&self) -> Solution {
        let mut rng = rand::thread_rng();

        let mut solution = Solution::new(&self.graph);
        let mut score = solution.score();

        let mut best_solution = solution.clone();
        let mut best_score = best_solution.score();

        let lim = 1000000;

        let mut temperture = 10.;
        let temperture_decay_rate = 0.9999996;

        for i in 0..lim {
            let prev_solution = solution.clone();
            solution.swap_2_opt();
            let new_score = solution.score();
            if new_score < best_score {
                best_solution = solution.clone();
                best_score = new_score;
                println!(
                    "best update. Current best score: {}, temperture: {:e}",
                    best_score, temperture
                );
            }

            let delta = new_score - score;
            let delta = delta as f64;

            // let random_number: f32 = rng.gen();
            if delta <= 0. {
                score = new_score;
            } else if rng.gen::<f64>() < (-delta / temperture).exp() {
                score = new_score;
                println!("Accepted bad transition. loop {}. score: {}, temperture: {:e}", i, score, temperture);
            } else {
                solution = prev_solution;
            }
            temperture *= temperture_decay_rate;
        }

        best_solution
    }
}
