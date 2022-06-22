from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        if len(nums) == 0:
            return []
        nums.sort()
        result = []
        first_num = nums[0] - 1  # set to diff value
        for ind_outer in range(len(nums) - 2):
            seen = set()  # Stores the 2nd numbers we've seen
            if first_num == nums[ind_outer]:
                continue  # Skip if same value for starting number
            first_num = nums[ind_outer]
            target = -first_num
            for num in nums[ind_outer + 1:]:
                diff = target - num
                if diff in seen:
                    result.append([first_num, diff, num])
                seen.add(num)
        return result


def tester():
    examples = [
        # Eg([-1, 0, 1, 2, -1, -4], [[-1, -1, 2], [-1, 0, 1]]),
        Eg([], []),
        Eg([0], []),
        Eg([0, 0, 0], [[0, 0, 0]])
    ]
    tester_helper(15, examples, Solution().threeSum)
