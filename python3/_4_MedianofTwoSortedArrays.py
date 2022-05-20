from typing import List

from python3.helper import Eg, tester_helper


# Src: https://youtu.be/q6IEA26hvXc

class Solution:
    def findMedianSortedArrays(self, nums1: List[int],
                               nums2: List[int]) -> float:
        merged: List[int] = []

        if len(nums1) == 0 and len(nums2) == 0:
            return 0
        elif len(nums1) == 0:
            merged = nums2
        elif len(nums2) == 0:
            merged = nums1
        else:
            # Both not 0
            i1 = 0
            i2 = 0
            while i1 < len(nums1) and i2 < len(nums2):
                if nums1[i1] < nums2[i2]:
                    merged.append(nums1[i1])
                    i1 += 1
                else:
                    merged.append(nums2[i2])
                    i2 += 1
            if i1 < len(nums1):
                assert i2 == len(nums2)
                for i in range(i1, len(nums1)):
                    merged.append(nums1[i1])
            elif i2 < len(nums2):
                for i in range(i2, len(nums2)):
                    merged.append(nums2[i2])
            assert len(nums1) + len(nums2) == len(merged)

        n = len(merged)
        if n % 2 == 0:
            return sum(merged[n // 2 - 1:n // 2 + 1]) / 2
        else:
            return float(merged[n // 2])


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
