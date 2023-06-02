use std::cmp::max;

struct Graph {
    /// Node numbers index into the list of their edges
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(node_count: usize) -> Self {
        Self {
            edges: vec![vec![]; node_count],
        }
    }

    fn add_edge(&mut self, src: usize, dst: usize) {
        debug_assert!(!self.edges[src].contains(&dst));
        self.edges[src].push(dst);
    }

    fn explosion_chain(&self, start_node: usize) -> i32 {
        self.explosion_chain_helper(start_node, &mut vec![false; self.len()])
    }

    fn explosion_chain_helper(&self, start_node: usize, is_visited: &mut [bool]) -> i32 {
        if is_visited[start_node] {
            return 0;
        } else {
            is_visited[start_node] = true
        }

        let mut result = 1;
        for &neighbour in self.edges[start_node].iter() {
            result += self.explosion_chain_helper(neighbour, is_visited);
        }
        result
    }

    fn len(&self) -> usize {
        self.edges.len()
    }
}

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        // Build Graph
        let mut graph = Graph::new(bombs.len());
        for (this_ind, this_bomb) in bombs.iter().enumerate() {
            let (this_x, this_y, this_blast_radius) = (
                this_bomb[0] as f32,
                this_bomb[1] as f32,
                this_bomb[2] as f32,
            );
            for (other_ind, other_bomb) in bombs.iter().enumerate().skip(this_ind + 1) {
                let (other_x, other_y, other_blast_radius) = (
                    other_bomb[0] as f32,
                    other_bomb[1] as f32,
                    other_bomb[2] as f32,
                );
                let distance = ((this_x - other_x).powf(2.0) + (this_y - other_y).powf(2.0)).sqrt();
                if distance <= this_blast_radius {
                    graph.add_edge(this_ind, other_ind);
                }
                if distance <= other_blast_radius {
                    graph.add_edge(other_ind, this_ind);
                }
            }
        }

        // Find node with longest chain
        let mut result = 1;
        for i in 0..graph.len() {
            result = max(result, graph.explosion_chain(i));
        }
        result
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![[2,1,3],[6,1,4]], 2)]
    #[case(vec![[1,1,5],[10,10,5]], 1)]
    #[case(vec![[1,2,3],[2,3,1],[3,4,2],[4,5,3],[5,6,4]], 5)]
    #[case(vec![
            [85024,58997,3532],[65196,42043,9739],[85872,75029,3117],[73014,91183,7092],
            [29098,40864,7624],[11469,13607,4315],[98722,69681,9656],[75140,42250,421],
            [92580,44040,4779],[58474,78273,1047],[27683,4203,6186],[10714,24238,6243],
            [60138,81791,3496],[16227,92418,5622],[60496,64917,2463],[59241,62074,885],
            [11961,163,5815],[37757,43214,3402],[21094,98519,1678],[49368,22385,1431],
            [6343,53798,159],[80129,9282,5139],[69565,32036,6827],[59372,64978,6575],
            [44948,71199,7095],[46390,91701,1667],[37144,98691,8128],
            [13558,81505,4653],[41234,48161,9304],[14852,3206,5369]
        ],
        3)]
    fn case(#[case] input: Vec<[i32; 3]>, #[case] expected: i32) {
        let input = input.into_iter().map(|x| x.into()).collect();
        let actual = Solution::maximum_detonation(input);
        assert_eq!(actual, expected);
    }
}
