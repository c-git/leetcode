from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def findKthLargest(self, nums: List[int], k: int) -> int:
        return sorted(nums, reverse=True)[k - 1]


def tester():
    examples = [
        Eg(([3, 2, 1, 5, 6, 4], 2), 5),
        Eg(([3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4),
    ]
    tester_helper(215, examples, Solution().findKthLargest)
