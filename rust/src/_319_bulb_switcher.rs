impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        let n = n as usize;
        let mut bulbs = vec![false; n];
        for step in 1..=n {
            for bulb_index in (0..n).skip(step - 1).step_by(step) {
                bulbs[bulb_index] = !bulbs[bulb_index];
            }
            if cfg!(debug_assertions) {
                println!("step = {step}. Bulbs = {bulbs:?}")
            }
        }
        if cfg!(debug_assertions) {
            println!("n = {n}. Bulbs = {bulbs:?}")
        }
        bulbs.iter().map(|x| if *x { 1 } else { 0 }).sum()
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
