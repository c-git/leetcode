from math import inf
from typing import List

from python3.helper import Eg, tester_helper


# Src: https://youtu.be/q6IEA26hvXc
# Src: https://leetcode.com/problems/median-of-two-sorted-arrays/discuss
# /1874742/Python-O(log(m%2Bn))-Modifed-Binary-Search-w-Comments

class Solution:
    def findMedianSortedArrays(self, nums1: List[int],
                               nums2: List[int]) -> float:

        if len(nums1) == 0 and len(nums2) == 0:
            return 0
        total = len(nums1) + len(nums2)
        half = total // 2

        # make sure nums1 is always smaller array
        if len(nums2) < len(nums1):
            nums1, nums2 = nums2, nums1

        left = 0
        right = len(nums1) - 1
        while True:  # no condition because there is guaranteed to be a
            # median so we can just return right away
            mid1 = left + (right - left) // 2
            mid2 = half - mid1 - 2  # -2 for 0 based array

            left1 = nums1[mid1] if mid1 >= 0 else -inf
            right1 = nums1[mid1 + 1] if (mid1 + 1) < len(nums1) else inf
            left2 = nums2[mid2] if mid2 >= 0 else -inf
            right2 = nums2[mid2 + 1] if (mid2 + 1) < len(nums2) else inf

            # If end of num1's left partition is smaller than right
            # partition num2's start, and vice versa for num2 and num1,
            # we have a valid partition, so then we compute result and return it
            if left1 <= right2 and left2 <= right1:
                # if we have odd length of array
                if total % 2 != 0:
                    # median is the beginning of  right partition, and it will
                    # be min value between right1 and right2
                    return min(right1, right2)

                # even length of array
                # median is the mean of two values adjacent to mid,
                # which are end of left partition and beginning of right
                # partition
                return (max(left1, left2) + min(right1, right2)) / 2
            # If end num1's left partition is larger than num2's start of
            # right partition, we need to fix partitions.
            # Since arrays are in ascending order, shifting right will result
            # in smaller num1's left partition, which leads to smaller left1
            elif left1 > right2:
                right = mid1 - 1
            # Or we could have too small 1, in which case we increase 1's
            # size by shifting left
            else:
                left = mid1 + 1


def tester():
    examples = [
        Eg(([1, 3], [2]), 2.0),
        Eg(([1, 2], [3, 4]), 2.5),
        Eg(([1], [2, 3]), 2.0),
        Eg(([1, 2, 3], []), 2.0),
        Eg(([], [1, 2, 3]), 2.0),
        Eg(([], [1]), 1.0),
        Eg(([1], []), 1.0),
        Eg(([1], [2]), 1.5),
        Eg(([], []), 0),
    ]
    tester_helper(4, examples, Solution().findMedianSortedArrays)
