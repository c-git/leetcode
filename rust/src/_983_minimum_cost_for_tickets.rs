impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        debug_assert!(costs.len() == 3, "As per constraint in question");
        let costs = vec![(1, costs[0]), (7, costs[1]), (30, costs[2])];
        Self::mincost_tickets_helper(&days[..], &costs)
    }

    fn mincost_tickets_helper(days: &[i32], costs: &[(i32, i32)]) -> i32 {
        if days.is_empty() {
            return 0;
        }
        let mut result = i32::MAX;
        let first_day = &days[0];
        for (duration, cost) in costs.iter() {
            let mut covered_days = days.len(); // Assume all are covered then loop to see if that does not hold (if not found then it does hold)
            for (i, day) in days.iter().enumerate() {
                if day >= &(first_day + duration) {
                    covered_days = i;
                    break;
                }
            }
            let option_cost = *cost + Self::mincost_tickets_helper(&days[covered_days..], costs);
            if result > option_cost {
                println!("days: {days:?}, duration: {duration}, covered days: {covered_days}, days left: {:?}, option cost: {option_cost}",&days[covered_days..]);
            }
            result = result.min(option_cost);
        }
        result
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
}
