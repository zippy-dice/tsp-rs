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

    pub fn solve(&self, params: &Params) -> Solution {
        let mut rng = rand::thread_rng();

        let mut solution = Solution::new(&self.graph);
        let mut score = solution.score();

        let mut best_solution = solution.clone();
        let mut best_score = best_solution.score();

        let lim = params.loops;

        let mut temperture = params.init_temperture;
        let temperture_decay_rate = params.temperture_decay_rate;

        for i in 0..lim {
            let prev_solution = solution.clone();
            Self::transition(&mut solution);
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
            // } else if rng.gen::<f64>() < (-delta / temperture).exp() {
            } else if rng.gen_bool((-delta / temperture).exp()) {
                score = new_score;
                println!(
                    "Accepted bad transition. loop {}. score: {}, temperture: {:e}",
                    i, score, temperture
                );
            } else {
                solution = prev_solution;
            }
            temperture *= temperture_decay_rate;
        }

        best_solution
    }

    fn transition(solution: &mut Solution) {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            solution.swap_2_opt();
        } else {
            solution.swap_3_opt();
        }
    }
}

pub struct Params {
    loops: usize,
    init_temperture: f64,
    temperture_decay_rate: f64,
}

impl Params {
    pub fn new() -> Params {
        Params {
            loops: 10000,
            init_temperture: 1000.,
            temperture_decay_rate: 0.999,
        }
    }

    pub fn loops(mut self, loops: usize) -> Self {
        self.loops = loops;
        self
    }

    pub fn init_temperture(mut self, init_temperture: f64) -> Self {
        self.init_temperture = init_temperture;
        self
    }

    pub fn temperture_decay_rate(mut self, temperture_decay_rate: f64) -> Self {
        self.temperture_decay_rate = temperture_decay_rate;
        self
    }
}
