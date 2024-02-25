//! Solution for https://leetcode.com/problems/find-all-people-with-secret
//! 2092. Find All People With Secret

use std::collections::BTreeSet;

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

    fn reset(&mut self) {
        for (i, val) in self.components.iter_mut().enumerate() {
            *val = i;
        }
    }
}

impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let n = n as usize;
        let first_person = first_person as usize; // Use an way to find set in UnionFind that knows secret
        let mut result: BTreeSet<usize> = [0, first_person].into();
        let mut uf = UnionFind::new(n);

        // Change from i32 to usize
        let mut meetings: Vec<Vec<usize>> = meetings
            .into_iter()
            .map(|x| x.into_iter().map(|y| y as usize).collect())
            .collect();

        meetings.sort_unstable_by(|x, y| x[2].cmp(&y[2])); // Sort by time

        let mut iter = meetings.into_iter().peekable();
        while let Some(meeting) = iter.next() {
            let mut curr_meetings = vec![meeting];

            // Collect all the current meetings
            while Some(true) == iter.peek().map(|x| x[2] == curr_meetings[0][2]) {
                curr_meetings.push(iter.next().expect("just checked that it is some"));
            }

            // See if someone in the meeting knows the secret
            if curr_meetings
                .iter()
                .any(|x| result.contains(&x[0]) || result.contains(&x[1]))
            {
                // Spread the secret too all people connected
                uf.reset();

                // Merge into groups based on meetings
                curr_meetings
                    .iter()
                    .for_each(|meeting| uf.union(meeting[0], meeting[1]));

                // Merge all people that know the password
                result.iter().for_each(|&x| uf.union(first_person, x));

                // Now that all the people are that know the secret are in one group add them to result
                for x in 0..n {
                    if uf.is_same_set(first_person, x) {
                        result.insert(x);
                    }
                }
            }
        }
        result.into_iter().map(|x| x as i32).collect()
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
