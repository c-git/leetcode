from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def minimumTotal(self, triangle: List[List[int]]) -> int:
        # src: Beixuan
        last_row = triangle[0]
        for i in range(1, len(triangle)):
            curr_row = [triangle[i][0] + last_row[0]]
            for j in range(1, len(triangle[i]) - 1):
                curr_row.append(triangle[i][j] + min(last_row[j - 1:j + 1]))
            curr_row.append(triangle[i][-1] + last_row[-1])
            last_row = curr_row
        return min(last_row)


def tester():
    examples = [
        Eg([[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]], 11),
        Eg([[-10]], -10)
    ]
    tester_helper(120, examples, Solution().minimumTotal)
