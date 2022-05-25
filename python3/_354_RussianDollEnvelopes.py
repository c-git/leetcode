from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    @staticmethod
    def can_fit(inner: List[int], outer: List[int]) -> bool:
        return inner[0] < outer[0] and inner[1] < outer[1]

    def maxEnvelopes(self, envelopes: List[List[int]]) -> int:
        # sort by perimeter
        envelopes.sort(key=lambda x: x[0] + x[1])

        table = [1] * len(envelopes)  # Create table to store best values

        for outer in range(1, len(envelopes)):
            best = 0  # init to nothing can fit
            for i in range(1, outer + 1):
                inner = outer - i
                # search through smaller to see the one that can fit that
                # holds the most
                if self.can_fit(envelopes[inner], envelopes[outer]) and \
                        table[inner] > best:
                    best = table[inner]
            table[outer] = best + 1

        return max(table)


def tester():
    examples = [
        Eg([[5, 4], [6, 4], [6, 7], [2, 3]], 3),
        Eg([[1, 1], [1, 1], [1, 1]], 1),
    ]
    tester_helper(354, examples, Solution().maxEnvelopes)
