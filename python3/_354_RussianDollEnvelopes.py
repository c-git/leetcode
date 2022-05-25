from bisect import bisect_left
from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    # src: https://leetcode.com/problems/russian-doll-envelopes/discuss
    # /2071640/100-or-0-MS-or-WITH-ANIMATION-or-EPIC-MEME
    def maxEnvelopes(self, envelopes: List[List[int]]) -> int:
        # width is increasing, but if two widths are the same, the height is 
        # decreasing
        envelopes.sort(key=lambda x: (x[0], -x[1]))

        dp = []
        for _, height in envelopes:
            left = bisect_left(dp, height)
            if left == len(dp):
                dp.append(height)
            else:
                dp[left] = height
        return len(dp)


def tester():
    examples = [
        Eg([[5, 4], [6, 4], [6, 7], [2, 3]], 3),
        Eg([[1, 1], [1, 1], [1, 1]], 1),
    ]
    tester_helper(354, examples, Solution().maxEnvelopes)
