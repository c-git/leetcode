from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        seen = {}
        for i, num in enumerate(nums):
            diff = target - num
            if seen.get(diff) is not None:
                return [seen[diff], i]
            else:
                seen[num] = i
        return []  # Not found


def tester():
    examples = [
        Eg(([2, 7, 11, 15], 9), [0, 1]),
        Eg(([3, 2, 4], 6), [1, 2]),
        Eg(([3, 3], 6), [0, 1]),
    ]
    tester_helper(1, examples, Solution().twoSum)
