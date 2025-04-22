//! Solution for https://leetcode.com/problems/count-the-number-of-ideal-arrays
//! 2338. Count the Number of Ideal Arrays

impl Solution {
    /// Copied from Editorial
    /// See also explanation at https://leetcode.com/problems/count-the-number-of-ideal-arrays/solutions/6675762/explaining-the-editorial-by-kosievdmerwe-gvhh/
    /// Unable to fully grok, decided to abandon in the interest of time
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let max_n: usize = max_value as usize + 10; // One: Don't know why the 10 is added
        const MAX_PRIMES: usize = 15; // There are up to 15 prime factors [One: don't know where the 15 came from but
                                      // guessing it's related to 2^14 being more than 10^4]

        let mut sieve = vec![0; max_n]; // Minimum prime factor [One: based on code looks like max prime factor]
        for i in 2..max_n {
            if sieve[i] == 0 {
                for j in (i..max_n).step_by(i) {
                    sieve[j] = i as i32;
                }
            }
        }

        let mut prime_counts = vec![vec![]; max_n]; // List of prime factor counts
        for (i, prime_count) in prime_counts
            .iter_mut()
            .take(max_value as usize + 1)
            .enumerate()
            .skip(2)
        {
            let mut x = i;
            while x > 1 {
                let p = sieve[x] as usize;
                let mut cnt = 0;
                while x % p == 0 {
                    x /= p;
                    cnt += 1;
                }
                prime_count.push(cnt);
            }
        }

        let mut dp = vec![vec![0; MAX_PRIMES + 1]; n as usize + MAX_PRIMES + 1];
        dp[0][0] = 1;
        for i in 1..n as usize + MAX_PRIMES + 1 {
            dp[i][0] = 1;
            for j in 1..=i.min(MAX_PRIMES) {
                dp[i][j] = (dp[i - 1][j] + dp[i - 1][j - 1]) % MOD;
            }
        }

        let mut result = 0i64;
        let n = n as usize;
        for prime_count in prime_counts.iter().take(max_value as usize + 1).skip(1) {
            let mut mul = 1i64;
            for &prime_count_element in prime_count {
                mul = mul * dp[n + prime_count_element as usize - 1][prime_count_element as usize]
                    % MOD;
            }
            result = (result + mul) % MOD;
        }

        result as i32
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
