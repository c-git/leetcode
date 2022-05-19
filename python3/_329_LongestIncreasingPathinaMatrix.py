from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def longestIncreasingPath(self, matrix: List[List[int]]) -> int:
        m = len(matrix)
        n = len(matrix[0])

        # First value is longest increasing second is longest decreasing
        track: List[List[List[int]]] = \
            [[[1] * 2 for _ in range(n)] for __ in range(m)]
        inc, dec = 0, 1  # index of value in matrix
        longest = -1

        # Build list of longest values
        for r in range(m):  # row
            for c in range(n):  # col
                # Up
                if r > 0:
                    if matrix[r][c] != matrix[r - 1][c]:
                        d = (
                            inc
                            if matrix[r][c] < matrix[r - 1][c] else
                            dec
                        )
                        nd = (d + 1) % 2  # not same direction
                        track[r][c][d] = track[r - 1][c][d] + track[r][c][nd]

                # Left
                if c > 0:
                    if matrix[r][c] != matrix[r][c - 1]:
                        d = (
                            inc
                            if matrix[r][c] < matrix[r][c - 1] else
                            dec
                        )
                        nd = (d + 1) % 2  # not same direction
                        val = track[r][c - 1][d] + track[r][c][nd]
                        nd = d + 1 % 2  # not same direction
                        track[r][c][d] = max(val, track[r][c][d])
                longest = max(max(track[r][c]), longest)
        return longest


def tester():
    examples = [
        Eg([[9, 9, 4], [6, 6, 8], [2, 1, 1]], 4),
        Eg([[3, 4, 5], [3, 2, 6], [2, 2, 1]], 4),
        Eg([[7, 8, 9], [9, 7, 6], [7, 2, 3]], 6)
    ]
    tester_helper(329, examples, Solution().longestIncreasingPath)
