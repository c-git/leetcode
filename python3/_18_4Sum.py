from math import inf
from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def binary_search(self, nums: List[int], target: int) -> bool:
        """
        Returns true if the target is found

        src: https://www.geeksforgeeks.org/python-program-for-binary-search/
        :param nums: The sorted numbers to search
        :param target: The number to search for
        :return: True if number found else False
        """
        low = 0
        high = len(nums) - 1
        while low <= high:
            mid = (high + low) // 2
            if nums[mid] < target:
                # If target is greater, ignore left half
                low = mid + 1
            elif nums[mid] > target:
                # If target is smaller, ignore right half
                high = mid - 1
            else:
                # means target is present at mid
                return True
        # If we reach here, then the element was not present
        return False

    def anySum(self, nums: List[int], target: int, levels: int) \
            -> List[List[int]]:
        """
        Solves any sum level recursively

        NB: nums must be sorted
        :param nums: Sorted list of numbers to pick from
        :param target: The value that the number should add up to
        :param levels: The number of numbers to be included in the sum
        :return: List of solutions
        """
        if len(nums) == 0:
            return []
        assert levels >= 1
        if levels == 1:
            return [] if not self.binary_search(nums, target) else [[target]]
        result = []
        num = -inf
        for i in range(len(nums) - levels + 1):
            if num == nums[i]:
                continue
            num = nums[i]
            for x in self.anySum(nums[i + 1:], target - num, levels - 1):
                result.append([num] + x)
        return result

    def clean_out_useless_values(self, nums: List[int], target):
        # src: Beixuan
        if len(nums) < 4:
            return nums
        i = 0
        min_ = target - sum(nums[-3:])
        while i < len(nums) - 3:
            if nums[i] < min_:
                i += 1
            else:
                break

        nums = nums[i:]

        if len(nums) < 4:
            return nums
        i = len(nums) - 1
        max_ = target - sum(nums[:3])
        while i >= 3:
            if nums[i] > max_:
                i -= 1
            else:
                break
        return nums[:i + 1]

    def fourSum(self, nums: List[int], target: int) -> List[List[int]]:
        nums.sort()
        nums = self.clean_out_useless_values(nums, target)
        return self.anySum(nums, target, 4)


def tester():
    examples = [
        # Eg(([1, 0, -1, 0, -2, 2], 0),
        #    [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]],
        #    evaluator_sort_to_compare_two_deep),
        # Eg(([2, 2, 2, 2, 2], 8), [[2, 2, 2, 2]]),
        Eg(([2, 2, 2, 2, 2, 80], 8), [[2, 2, 2, 2]]),
        Eg(([-50, 2, 2, 2, 2, 2], 8), [[2, 2, 2, 2]]),
        Eg(([2, 2, 2, 2, 2, 2], 8), [[2, 2, 2, 2]]),
        Eg(([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0), [[0, 0, 0, 0]]),
    ]
    tester_helper(18, examples, Solution().fourSum)
