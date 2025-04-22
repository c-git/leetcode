//! Solution for https://leetcode.com/problems/count-the-number-of-ideal-arrays
//! 2338. Count the Number of Ideal Arrays

impl Solution {
    /// Copied from Editorial
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const MAX_N: usize = 10_000 + 10;
        const MAX_P: usize = 15; // There are up to 15 prime factors

        let mut sieve = vec![0; MAX_N]; // Minimum prime factor
        for i in 2..MAX_N {
            if sieve[i] == 0 {
                for j in (i..MAX_N).step_by(i) {
                    sieve[j] = i as i32;
                }
            }
        }

        let mut ps = vec![vec![]; MAX_N]; // List of prime factor counts
        for i in 2..=max_value as usize {
            let mut x = i;
            while x > 1 {
                let p = sieve[x] as usize;
                let mut cnt = 0;
                while x % p == 0 {
                    x /= p;
                    cnt += 1;
                }
                ps[i].push(cnt);
            }
        }

        let mut c = vec![vec![0; MAX_P + 1]; n as usize + MAX_P + 1];
        c[0][0] = 1;
        for i in 1..n as usize + MAX_P + 1 {
            c[i][0] = 1;
            for j in 1..=i.min(MAX_P) {
                c[i][j] = (c[i - 1][j] + c[i - 1][j - 1]) % MOD;
            }
        }

        let mut ans = 0i64;
        let n = n as usize;
        for x in 1..=max_value as usize {
            let mut mul = 1i64;
            for &p in &ps[x] {
                mul = mul * c[n + p as usize - 1][p as usize] % MOD;
            }
            ans = (ans + mul) % MOD;
        }

        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 5, 10)]
    #[case(5, 3, 11)]
    #[case(5, 9, 111)]
    #[case(184, 389, 510488787)]
    #[case(5878, 2900, 465040898)]
    #[case(9767, 9557, 1998089)]
    fn case(#[case] n: i32, #[case] max_value: i32, #[case] expected: i32) {
        let actual = Solution::ideal_arrays(n, max_value);
        assert_eq!(actual, expected);
    }
}
