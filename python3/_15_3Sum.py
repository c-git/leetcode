from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        result = []
        for ind_outer in range(len(nums) - 2):
            seen = set()  # Stores the 2nd numbers we've seen
            first_num = nums[ind_outer]
            target = -first_num
            for ind_inner, num in enumerate(nums[ind_outer + 1:]):
                if ind_inner == ind_outer:
                    continue
                diff = target - num
                if diff in seen:
                    result.append([first_num, diff, num])
                seen.add(num)
        return result


def tester():
    examples = [
        Eg([-1, 0, 1, 2, -1, -4], [[-1, -1, 2], [-1, 0, 1]]),
        Eg([], []),
        Eg([0], [])
    ]
    tester_helper(15, examples, Solution().threeSum)
