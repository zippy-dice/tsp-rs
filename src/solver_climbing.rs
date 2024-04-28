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
        const EPS: f32 = 1e-5;
        let n = self.graph.size;

        let mut sijs = vec![];

        for idx_i in 0..n {
            for idx_j in idx_i + 1..n {
                if idx_i == 0 && idx_j == n - 1 {
                    continue;
                }
                let diff = solution.diff_score_2_opt(idx_i, idx_j);
                if diff < -EPS {
                    sijs.push((diff, idx_i, idx_j));
                }
            }
        }

        sijs.sort_by(|left, right| left.0.partial_cmp(&right.0).unwrap());
        let mut res = false;

        for (_, i, j) in sijs {
            let diff = solution.diff_score_2_opt(i, j);
            if diff < -EPS {
                solution.swap_2_opt(i, j);
                res = true;
            }
        }

        res
    }
}
