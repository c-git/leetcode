from itertools import combinations
from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    NUMBERS = [x for x in range(1, 10)]

    def combinationSum3(self, k: int, n: int) -> List[List[int]]:
        result = []
        for x in combinations(self.NUMBERS, k):
            if sum(x) == n:
                result.append(list(x))
        return result


def tester():
    examples = [
        Eg((3, 7), [[1, 2, 4]]),
        Eg((3, 9), [[1, 2, 6], [1, 3, 5], [2, 3, 4]]),
        Eg((4, 1), []),
    ]
    tester_helper(216, examples, Solution().combinationSum3)
