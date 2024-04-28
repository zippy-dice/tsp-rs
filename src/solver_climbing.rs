use crate::graph::Graph;
use crate::solution::Solution;

pub struct SolverClimbing {
    graph: Graph,
}

impl SolverClimbing {
    pub fn from_graph(graph: Graph) -> SolverClimbing {
        SolverClimbing { graph }
    }

    pub fn climbing(&self, solution: &mut Solution) -> bool {
        let mut ij = None;
        let mut best_diff = -1e-5;
        let n = self.graph.size;

        for idx_i in 0..n {
            for idx_j in idx_i + 1..n {
                if idx_i == 0 && idx_j == n - 1 {
                    continue;
                }
                let diff = solution.diff_score_2_opt(idx_i, idx_j);
                if diff < best_diff {
                    best_diff = diff;
                    ij = Some((idx_i, idx_j));
                }
            }
        }

        if let Some((i, j)) = ij {
            solution.swap_2_opt(i, j);
            true
        } else {
            false
        }
    }
}
