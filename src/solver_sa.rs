use crate::graph::Graph;
use crate::solution::Solution;

pub struct SolverSA {
    graph: Graph,
}

impl SolverSA {
    pub fn from_graph(graph: Graph) -> SolverSA {
        SolverSA { graph }
    }

    pub fn solve(&self) -> Solution {
        let mut solution = Solution::new(&self.graph);
        let score = solution.score();

        let mut best_solution = solution.clone();
        let mut best_score = best_solution.score();

        let lim = 1000000;

        for _ in 0..lim {
            let prev_solution = solution.clone();
            solution.swap_2_opt();
            let new_score = solution.score();
            if new_score < best_score {
                best_solution = solution.clone();
                best_score = new_score;
                println!("best update. Current best score: {}", best_score);
            } else {
                solution = prev_solution;
            }
        }

        best_solution
    }
}
