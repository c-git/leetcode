//! Solution for https://leetcode.com/problems/first-bad-version
//! 278. First Bad Version

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut good = 0;
        let mut bad = n as u32;
        loop {
            // LI:
            //  - `good` is a good version
            //  -`bad` is a bad version
            //  - `good` < `bad`
            debug_assert!(!self.isBadVersion(good as _));
            debug_assert!(self.isBadVersion(bad as _));
            debug_assert!(good < bad);
            let mid = ((bad + 1u32 + good) / 2) as _; // +1 to round up
            if self.isBadVersion(mid) {
                if mid as u32 == bad {
                    // If `good` and `bad` are next to each other then because
                    // of round up `mid` will be equal to `bad`. And we know
                    // that this must happen because the search space shrinks
                    // each iteration and thus they must eventually be next to
                    // each other
                    return mid;
                }
                // Bad must be greater than mid so we are reducing the search space
                debug_assert!(bad > mid as _);
                bad = mid as _;
            } else {
                // Because of round up this will always reduce the size of the search space
                debug_assert!(good < mid as _);
                good = mid as _;
            }
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution {
    bad: i32,
}

impl Solution {
    #[allow(non_snake_case)]
    pub fn isBadVersion(&self, version: i32) -> bool {
        version >= self.bad
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, 4)]
    #[case(1, 1)]
    #[case(2126753390, 1702766719)]
    #[case(i32::MAX, i32::MAX)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let sol = Solution { bad: expected };
        let actual = Solution::first_bad_version(&sol, n);
        assert_eq!(actual, expected);
    }
}
