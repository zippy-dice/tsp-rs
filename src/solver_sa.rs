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
        let solution = Solution::new(&self.graph);
        todo!();
        solution
    }
}
