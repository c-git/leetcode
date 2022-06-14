from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    # src: LeetCode Solution
    def __init__(self):
        # Create variables passed to all functions
        self.s1 = None
        self.s2 = None
        self.memo = None

    def minDistance(self, word1: str, word2: str) -> int:
        self.s1 = word1
        self.s2 = word2
        m = len(word1)
        n = len(word2)
        self.memo: List[List[int]] = \
            [[-1] * (n + 1) for _ in range(m + 1)]
        return m + n - 2 * self.lcs(m, n)

    def lcs(self, m: int, n: int):
        if self.memo[m][n] >= 0:
            return self.memo[m][n]
        if m == 0 or n == 0:
            self.memo[m][n] = 0
            return 0
        if self.s1[m - 1] == self.s2[n - 1]:
            self.memo[m][n] = 1 + self.lcs(m - 1, n - 1)
        else:
            self.memo[m][n] = max(self.lcs(m, n - 1), self.lcs(m - 1, n))
        return self.memo[m][n]


def tester():
    examples = [
        Eg(("sea", "eat"), 2),
        Eg(("leetcode", "etco"), 4),
        Eg(('a', 'a'), 0),
    ]
    tester_helper(583, examples, Solution().minDistance)
