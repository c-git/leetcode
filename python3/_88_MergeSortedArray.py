import math
from typing import Any, List, Tuple

from python3.helper import Eg, tester_helper


class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        # Convert m and n into indices
        m -= 1
        n -= 1

        next_pos = len(nums1) - 1
        while n >= 0:
            # LI:
            # - All indices greater than m are already inserted into the
            # sorted part of nums1
            # - All indices greater than n in nums2 are already inserted into
            # the sorted part of nums 1
            # - All indices in nums1 greater than next_pos are already sorted
            assert m < next_pos
            a = nums1[m] if m >= 0 else -math.inf
            b = nums2[n] if n >= 0 else -math.inf
            if b >= a:
                nums1[next_pos] = b
                n -= 1
            else:
                nums1[next_pos] = a
                m -= 1
            next_pos -= 1


def ev(in_: Tuple[Any], out_, exp) -> bool:
    return in_[0] == exp


def tester():
    examples = [
        Eg(([1, 2, 3, 0, 0, 0], 3, [2, 5, 6], 3), [1, 2, 2, 3, 5, 6], ev),
        Eg(([1], 1, [], 0), [1], ev),
        Eg(([0], 0, [1], 1), [1], ev),
    ]
    tester_helper(88, examples, Solution().merge)
