impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        let n = n as usize;
        let mut bulbs = vec![0; n]; // Used 0 and 1 instead because they are the same length and print better in columns
        for step in 1..=n {
            for bulb_index in (0..n).skip(step - 1).step_by(step) {
                bulbs[bulb_index] = 1 - bulbs[bulb_index];
            }
            if cfg!(debug_assertions) {
                println!("step = {step}. Bulbs = {bulbs:?}")
            }
        }
        if cfg!(debug_assertions) {
            println!("n = {n}. Bulbs = {bulbs:?}")
        }
        bulbs.iter().sum()
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = 3;
        let expected = 1;
        let actual = Solution::bulb_switch(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case2() {
        let input = 0;
        let expected = 0;
        let actual = Solution::bulb_switch(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn case3() {
        let input = 1;
        let expected = 1;
        let actual = Solution::bulb_switch(input);
        assert_eq!(actual, expected);
    }
}
