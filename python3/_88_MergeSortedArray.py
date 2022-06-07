from typing import Any, List, Tuple

from python3.helper import Eg, tester_helper


class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        nums1[m:] = nums2
        nums1.sort()


def ev(in_: Tuple[Any], out_, exp) -> bool:
    return in_[0] == exp


def tester():
    examples = [
        Eg(([1, 2, 3, 0, 0, 0], 3, [2, 5, 6], 3), [1, 2, 2, 3, 5, 6], ev),
        Eg(([1], 1, [], 0), [1], ev),
        Eg(([0], 0, [1], 1), [1], ev),
    ]
    tester_helper(88, examples, Solution().merge)
