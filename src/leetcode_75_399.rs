use std::collections::{HashMap, HashSet};

use crate::Solution;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        //  build the adjacency matrix
        let mut graph = HashMap::new();
        for (index, equation) in equations.iter().enumerate() {
            graph
                .entry(equation[0].clone())
                .or_insert(Vec::new())
                .push((equation[1].clone(), values[index]));
            graph
                .entry(equation[1].clone())
                .or_insert(Vec::new())
                .push((equation[0].clone(), 1.0 / values[index]));
        }

        fn dfs(
            node: (String, f64),
            destination: String,
            mut result: f64,
            answers: &mut Vec<f64>,
            graph: &HashMap<String, Vec<(String, f64)>>,
            visited: &mut HashSet<String>,
        ) {
            if visited.contains(&node.0) {
                return;
            }
            visited.insert(node.0.clone());
            result = result * node.1;
            if node.0 == destination {
                answers.push(result);
                return;
            }
            if let Some(neighbours) = graph.get(&node.0) {
                for neighbour in neighbours {
                    dfs(
                        neighbour.clone(),
                        destination.clone(),
                        result,
                        answers,
                        graph,
                        visited,
                    );
                }
            }
        }
        let mut output = Vec::new();
        for query in queries {
            let mut answers = Vec::new();
            if !graph.contains_key(&query[0]) {
                output.push(-1.0);
                continue;
            }
            dfs(
                (query[0].clone(), 1.0),
                query[1].clone(),
                1.0,
                &mut answers,
                &graph,
                &mut HashSet::new(),
            );
            if answers.len() == 0 || answers.len() > 1 {
                output.push(-1.0);
            } else {
                output.push(answers[0]);
            }
        }
        output
    }
}
