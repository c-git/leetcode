//! Solution for https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign
//! 2071. Maximum Number of Tasks You Can Assign

use std::collections::VecDeque;

impl Solution {
    /// Based on https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign/solutions/6704796/beats-100-in-mosteasy-solution-usingbina-ucot/
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        tasks.sort_unstable();
        workers.sort_unstable();

        let (mut low, mut high) = (0, tasks.len().min(workers.len()));
        while low < high {
            let mid = (low + high).div_ceil(2);
            if can_assign(&tasks, &workers, pills, strength, mid) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        low as i32
    }
}

fn can_assign(tasks: &[i32], workers: &[i32], mut pills: i32, strength: i32, k: usize) -> bool {
    let mut available_workers: VecDeque<_> = workers[workers.len() - k..].to_vec().into();

    for &task in tasks.iter().take(k).rev() {
        if let Some(&w) = available_workers.back() {
            if w >= task {
                available_workers.pop_back();
                continue;
            }
        }
        if pills == 0 {
            return false;
        }

        let first_able_worker =
            available_workers.partition_point(|&worker| worker + strength < task);
        if first_able_worker < available_workers.len() {
            available_workers.remove(first_able_worker);
            pills -= 1;
        } else {
            return false;
        }
    }
    true
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,2,1], vec![0,3,3], 1, 1, 3)]
    #[case(vec![5,4], vec![0,0,0], 1, 5, 1)]
    #[case(vec![10,15,30], vec![0,10,10,10,10], 3, 10, 2)]
    #[case(vec![5,9,8,5,9], vec![1,6,4,2,6], 1, 5, 3)]
    fn case(
        #[case] tasks: Vec<i32>,
        #[case] workers: Vec<i32>,
        #[case] pills: i32,
        #[case] strength: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::max_task_assign(tasks, workers, pills, strength);
        assert_eq!(actual, expected);
    }
}
