impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        // Use DP and calculate best solution for each index starting from the latest day and going forward,
        // then can use previously calculated solutions to calculate the one before it until we get to the start then we've solved it
        debug_assert!(costs.len() == 3, "As per constraint in question");
        let costs = vec![(1, costs[0]), (7, costs[1]), (30, costs[2])];

        // Create table with best values and initialize all values to 0
        let mut table = vec![i32::MAX; days.len() + 1]; // Plus one for the set with no days
        table[days.len()] = 0; // Initialize last value to 0 as there are no days and thus no cost

        for day in (0..days.len()).rev() {
            let first_date = &days[day];
            for (duration, cost) in costs.iter() {
                // Assume all are covered then loop to see if that does not hold (if not found then it does hold).
                // days.len() is valid index because there is an extra slot with a 0 to be used as a base case
                let mut next_uncovered_index = days.len();
                for (i, date) in days[day..].iter().enumerate() {
                    if date >= &(first_date + duration) {
                        next_uncovered_index = i + day;
                        break;
                    }
                }
                let option_cost = *cost + table[next_uncovered_index];
                table[day] = table[day].min(option_cost);
            }
        }

        table[0] // Question constraint guarantees that there will be at least one day in the input
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let days = vec![1, 4, 6, 7, 8, 20];
        let costs = vec![2, 7, 15];
        let expected = 11;
        let actual = Solution::mincost_tickets(days, costs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31];
        let costs = vec![2, 7, 15];
        let expected = 17;
        let actual = Solution::mincost_tickets(days, costs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let days = vec![
            3, 5, 6, 8, 11, 13, 15, 16, 17, 18, 19, 20, 24, 27, 30, 31, 33, 36, 37, 40, 45, 48, 51,
            54, 56, 57, 61, 62, 64, 66, 67, 71, 73, 74, 77, 78, 80, 81, 83, 84, 86, 87, 90, 91, 94,
            96, 97,
        ];
        let costs = vec![3, 16, 66];
        let expected = 139;
        let actual = Solution::mincost_tickets(days, costs);
        assert_eq!(actual, expected);
    }
}
