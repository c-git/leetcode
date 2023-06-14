impl Solution {
    /// Intuition - Any vertex: with a parent is not a root. Doesn't matter how many nodes point to it.
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        // usize in this code is assumed to be u64
        assert_eq!(usize::max_value() as u64, u64::max_value());
        let mut result = vec![];
        let num_bit_groups = (n as usize + 64 - 1) / 64; // Divide by 64 rounding up to get how many u64's we need to store all the values seen
        let mut seen = vec![0; num_bit_groups];

        // Check which vertices have edges leading to them
        for edge in edges {
            let to_vertex = edge[1] as usize;
            let group_index = to_vertex / 64;
            let bit_index = to_vertex as u32 % 64;
            let or_val = usize::pow(2, bit_index);
            seen[group_index] |= or_val;
        }

        // Check which vertices were not seen. These are the roots as they don't have a parent
        let mut group_index = 0;
        let mut and_val = 1;
        for _ in 0..n {
            if seen[group_index] & and_val == 0 {
                let vertex_number = group_index as i32 * 64 + and_val.trailing_zeros() as i32;
                result.push(vertex_number)
            }
            and_val <<= 1;
            if and_val == 0 {
                and_val = 1;
                group_index += 1;
            }
        }

        result
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    fn validate(mut actual: Vec<i32>, mut expected: Vec<i32>) {
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn case1() {
        let n = 6;
        let edges = [[0, 1], [0, 2], [2, 5], [3, 4], [4, 2]];
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = [0, 3];
        let actual = Solution::find_smallest_set_of_vertices(n, edges);
        validate(actual, expected.into());
    }

    #[test]
    fn case2() {
        let n = 5;
        let edges = [[0, 1], [2, 1], [3, 1], [1, 4], [2, 4]];
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = [0, 2, 3];
        let actual = Solution::find_smallest_set_of_vertices(n, edges);
        validate(actual, expected.into());
    }

    #[test]
    fn case3() {
        let n = 6;
        let edges = [[0, 1], [0, 2], [2, 5], [3, 4], [4, 2], [0, 3]];
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = [0];
        let actual = Solution::find_smallest_set_of_vertices(n, edges);
        validate(actual, expected.into());
    }

    #[test]
    fn case4() {
        let n = 6;
        let edges = [[1, 5], [0, 1]];
        let edges = edges.into_iter().map(|x| x.into()).collect();
        let expected = [0, 2, 3, 4];
        let actual = Solution::find_smallest_set_of_vertices(n, edges);
        validate(actual, expected.into());
    }
}
