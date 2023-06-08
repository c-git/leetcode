use std::collections::{hash_map::Entry, HashMap, VecDeque};

use RelationshipType as RelType;

#[derive(PartialEq)]
enum RelationshipType {
    Numerator,
    Denominator,
}

struct Edge {
    dst_node_id: usize,
    src_rel_type: RelationshipType,
    value: f64,
}

impl Edge {
    fn get_value(&self, src_type: RelationshipType) -> f64 {
        if src_type == self.src_rel_type {
            self.value
        } else {
            1.0 / self.value
        }
    }
}

struct SymbolMap {
    names_to_id: HashMap<String, usize>,
    edges: Vec<Vec<Edge>>,
}

impl SymbolMap {
    fn new(equations: Vec<Vec<String>>, values: Vec<f64>) -> Self {
        let mut result = Self {
            names_to_id: Default::default(),
            edges: Default::default(),
        };

        for (equation, val) in equations.into_iter().zip(values.into_iter()) {
            result.add_edge(equation, val);
        }

        result
    }

    fn calc(&self, mut query: Vec<String>) -> Option<f64> {
        let denominator_name = query.pop().unwrap();
        let numerator_name = query.pop().unwrap();
        let (numerator_id, denominator_id) = if let (Some(numerator), Some(denominator)) =
            (self.get_id(numerator_name), self.get_id(denominator_name))
        {
            (numerator, denominator)
        } else {
            // At least one is not known
            return None;
        };

        // Do BFS to find other side
        let mut queue = VecDeque::new();
        let mut is_visited = vec![false; self.names_to_id.len()];
        queue.push_back((numerator_id, 1.0));
        is_visited[numerator_id] = true;
        while let Some((node_id, val)) = queue.pop_front() {
            if node_id == denominator_id {
                return Some(val);
            }

            for edge in self.edges[node_id].iter() {
                if !is_visited[edge.dst_node_id] {
                    is_visited[edge.dst_node_id] = true;
                    queue.push_back((edge.dst_node_id, val * edge.get_value(RelType::Numerator)));
                }
            }
        }

        None
    }

    fn add_edge(&mut self, mut equation: Vec<String>, val: f64) {
        let denominator_name = equation.pop().unwrap();
        let numerator_name = equation.pop().unwrap();
        let denominator_id = self.get_or_create_id(denominator_name);
        let numerator_id = self.get_or_create_id(numerator_name);
        self.edges[numerator_id].push(Edge {
            dst_node_id: denominator_id,
            src_rel_type: RelType::Numerator,
            value: val,
        });
        self.edges[denominator_id].push(Edge {
            dst_node_id: numerator_id,
            src_rel_type: RelType::Denominator,
            value: val,
        });
    }

    fn get_id(&self, name: String) -> Option<usize> {
        self.names_to_id.get(&name).map(|x| *x)
    }

    fn get_or_create_id(&mut self, name: String) -> usize {
        match self.names_to_id.entry(name) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let result = self.edges.len();
                entry.insert(result);
                self.edges.push(Vec::new()); // Add new spot for new node
                result
            }
        }
    }
}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let symbol_map = SymbolMap::new(equations, values);
        queries
            .into_iter()
            .map(|q| symbol_map.calc(q).unwrap_or(-1.0))
            .collect()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![["a","b"],["b","c"]], vec![2.0,3.0], vec![["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]], vec![6.00000,0.50000,-1.00000,1.00000,-1.00000])]
    #[case(vec![["a","b"],["b","c"],["bc","cd"]], vec![1.5,2.5,5.0], vec![["a","c"],["c","b"],["bc","cd"],["cd","bc"]],vec![3.75000,0.40000,5.00000,0.20000])]
    #[case(vec![["a","b"]], vec![0.5], vec![["a","b"],["b","a"],["a","c"],["x","y"]], vec![0.50000,2.00000,-1.00000,-1.00000])]
    fn case(
        #[case] equations: Vec<[&str; 2]>,
        #[case] values: Vec<f64>,
        #[case] queries: Vec<[&str; 2]>,
        #[case] expected: Vec<f64>,
    ) {
        let equations = equations
            .into_iter()
            .map(|x| x.iter().map(|y| y.to_string()).collect())
            .collect();
        let queries = queries
            .into_iter()
            .map(|x| x.iter().map(|y| y.to_string()).collect())
            .collect();
        let actual = Solution::calc_equation(equations, values, queries);
        assert_eq!(actual, expected);
    }
}
