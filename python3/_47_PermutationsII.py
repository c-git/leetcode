from itertools import permutations
from typing import List

from python3.helper import Eg, evaluator_any_order_list, tester_helper


class Solution:
    def permuteUnique(self, nums: List[int]) -> List[List[int]]:
        result = set()
        for x in permutations(nums):
            result.add(''.join([str(i) for i in x]))
        return [[int(i) for i in x] for x in result]


def tester():
    examples = [
        Eg([1, 1, 2],
           [[1, 1, 2], [1, 2, 1], [2, 1, 1]], evaluator_any_order_list),
        Eg([1, 2, 3],
           [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]],
           evaluator_any_order_list),
    ]
    tester_helper(47, examples, Solution().permuteUnique)
