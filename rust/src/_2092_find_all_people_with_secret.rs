//! Solution for https://leetcode.com/problems/find-all-people-with-secret
//! 2092. Find All People With Secret

use std::collections::BTreeMap;

struct UnionFind {
    components: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            components: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.components[x] {
            self.components[x] = self.find(self.components[x]);
        }
        self.components[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                std::mem::swap(&mut root_x, &mut root_y)
            }
            self.rank[root_y] += self.rank[root_x];
            self.components[root_x] = root_y;
        }
    }

    fn is_same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn reset(&mut self, x: usize) {
        self.components[x] = x;
        self.rank[x] = 1;
    }
}

impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        // Based on last version in Editorial
        let n = n as usize;
        let first_person = first_person as usize;

        // Sort meetings in increasing order of time
        meetings.sort_unstable_by_key(|meeting| meeting[2]);

        // Group Meetings in increasing order of time
        let mut same_time_meetings: BTreeMap<i32, Vec<(i32, i32)>> = BTreeMap::new();
        for meeting in meetings {
            let [x, y, t] = meeting[..] else {
                unreachable!("guaranteed to be 3 by constraint")
            };
            same_time_meetings.entry(t).or_default().push((x, y));
        }

        // Create graph
        let mut graph = UnionFind::new(n);
        graph.union(first_person, 0);

        // Process in increasing order of time
        for meetings_at_time in same_time_meetings.values() {
            // Unite two persons taking part in a meeting
            for (x, y) in meetings_at_time.iter() {
                graph.union(*x as usize, *y as usize);
            }

            // If any one knows the secret, both will be connected to 0.
            // If no one knows the secret, then reset.
            for (x, y) in meetings_at_time.iter() {
                if !graph.is_same_set(*x as usize, 0) {
                    // No need to check for y since x and y were united
                    graph.reset(*x as usize);
                    graph.reset(*y as usize);
                }
            }
        }

        // All those who are connected to 0 will know the secret
        (0..n)
            .filter_map(|x| {
                if graph.is_same_set(x, 0) {
                    Some(x as i32)
                } else {
                    None
                }
            })
            .collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(6, vec![vec![1,2,5],vec![2,3,8],vec![1,5,10]], 1, vec![0,1,2,3,5])]
    #[case(4, vec![vec![3,1,3],vec![1,2,2],vec![0,3,3]], 3, vec![0,1,3])]
    #[case(5, vec![vec![3,4,2],vec![1,2,1],vec![2,3,1]], 1, vec![0,1,2,3,4])]
    #[case(6, vec![vec![0,2,1],vec![1,3,1],vec![4,5,1]], 1, vec![0,1,2,3,])]
    #[case(5, vec![vec![1,4,3],vec![0,4,3]], 3, vec![0,1,3,4])]

    fn case(
        #[case] n: i32,
        #[case] meetings: Vec<Vec<i32>>,
        #[case] first_person: i32,
        #[case] mut expected: Vec<i32>,
    ) {
        let mut actual = Solution::find_all_people(n, meetings, first_person);
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
