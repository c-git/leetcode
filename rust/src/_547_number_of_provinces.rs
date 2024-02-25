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
}

impl Solution {
    pub fn find_circle_num(mut is_connected: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let n = is_connected.len();
        let mut uf = UnionFind::new(n);
        for (index1, city1) in is_connected.iter().enumerate().take(n - 1) {
            for (index2, are_connected) in city1.iter().enumerate().skip(index1 + 1) {
                if *are_connected == 1 {
                    uf.union(index1, index2);
                }
            }
        }

        // Reusing first row of is_connected instead of allocating new memory (but would be the same if new vec was allocated) (Using the value -1 as an indicator of seen before)

        let unique_values = &mut is_connected[0];
        for i in 0..n {
            let representative_city = uf.find(i);
            if unique_values[representative_city] != -1 {
                // Count the number of times we find a new representative city
                unique_values[representative_city] = -1;
                result += 1;
            }
        }
        result
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]], 2)]
    #[case(vec![vec![1,0,0],vec![0,1,0],vec![0,0,1]], 3)]
    fn case(#[case] input: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::find_circle_num(input);
        assert_eq!(actual, expected);
    }
}
