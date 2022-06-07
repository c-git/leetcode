from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def maxArea(self, height: List[int]) -> int:
        result = 0
        for i in range(1, len(height)):
            # TODO: Add early break if value cannot get better
            curr = height[i]
            for j in range(0, i):
                vol = min(curr, height[j]) * (i - j)
                if vol > result:
                    result = vol
        return result


def tester():
    examples = [
        Eg([1, 8, 6, 2, 5, 4, 8, 3, 7], 49),
        Eg([1, 1], 1),
    ]
    tester_helper(11, examples, Solution().maxArea)
