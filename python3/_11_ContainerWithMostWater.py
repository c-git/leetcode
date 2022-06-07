from typing import List

from python3.helper import Eg, tester_helper


# src: https://leetcode.com/problems/container-with-most-water/discuss
# /2119019/100-Fastest-Solution-Explained
class Solution:
    def maxArea(self, height: List[int]) -> int:
        result = 0
        left = 0
        right = len(height) - 1
        while left < right:
            if height[left] < height[right]:
                side_height = height[left]
                left += 1
            else:
                side_height = height[right]
                right -= 1
            vol = side_height * (right - left + 1)
            if vol > result:
                result = vol
        return result


def tester():
    examples = [
        Eg([1, 8, 6, 2, 5, 4, 8, 3, 7], 49),
        Eg([1, 1], 1),
    ]
    tester_helper(11, examples, Solution().maxArea)
