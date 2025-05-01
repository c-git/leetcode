//! Solution for https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign
//! 2071. Maximum Number of Tasks You Can Assign

use std::collections::HashMap;

impl Solution {
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        tasks.sort_unstable();
        workers.sort_unstable();
        let mut memo = HashMap::new();

        Self::max_task_assign_(&tasks, &workers, pills, strength, 0, &mut memo)
    }

    /// Returns the maximum number of tasks left than can be completed with the resources left
    fn max_task_assign_(
        tasks: &[i32],
        workers: &[i32],
        pills: i32,
        strength: i32,
        already_done: i32,
        memo: &mut HashMap<(usize, usize, i32), i32>,
    ) -> i32 {
        if tasks.is_empty() || workers.is_empty() {
            return already_done;
        }
        let key = (tasks.len(), workers.len(), pills);
        if let Some(&result) = memo.get(&key) {
            return result;
        }
        let is_pills_left = pills > 0;
        let result = match (tasks[0], workers[0], is_pills_left) {
            (task, worker, _) if task <= worker => {
                // Always take this option as any later worker can do any task this worker could also have done
                Self::max_task_assign_(
                    &tasks[1..],
                    &workers[1..],
                    pills,
                    strength,
                    already_done + 1,
                    memo,
                )
            }
            (task, worker, true) if task <= worker + strength => {
                // We can use a pill here to solve a task, check both using the pill and not using it
                let use_pill_for_task = Self::max_task_assign_(
                    &tasks[1..],
                    &workers[1..],
                    pills - 1,
                    strength,
                    already_done + 1,
                    memo,
                );
                let without_pill_discard_worker = Self::max_task_assign_(
                    tasks,
                    &workers[1..],
                    pills,
                    strength,
                    already_done,
                    memo,
                );
                use_pill_for_task.max(without_pill_discard_worker)
            }
            _ => {
                // This worker cannot be assigned any tasks
                Self::max_task_assign_(tasks, &workers[1..], pills, strength, already_done, memo)
            }
        };
        memo.insert(key, result);
        result
    }
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
