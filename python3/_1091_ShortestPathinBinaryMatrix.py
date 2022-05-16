from copy import deepcopy
from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def shortestPathBinaryMatrix(self, grid: List[List[int]]) -> int:
        # Needed for timing as input cannot be changed. Remove before submission
        grid = deepcopy(grid)

        # Src: Modified from Beixuan's solution
        if grid[0][0] or grid[-1][-1]:
            return -1

        n = len(grid) - 1  # Last index value (Assumes square input)
        reached = [(1, 0, 0)]  # (dist, x, y)
        while len(reached):
            dist, x, y = reached.pop(0)
            hor_min = max(0, x - 1)
            hor_max = min(n, x + 1)
            ver_min = max(0, y - 1)
            ver_max = min(n, y + 1)
            for hor in range(hor_min, hor_max + 1):
                for ver in range(ver_min, ver_max + 1):
                    if hor == n and ver == n:
                        return dist + 1
                    if hor != x or ver != y:
                        # On a neighbour
                        if grid[hor][ver] == 0:
                            grid[hor][ver] = 1  # Reached in fastest way already
                            reached.append((dist + 1, hor, ver))
            reached.sort()
        return -1


def tester():
    examples = [
        Eg([[0, 1], [1, 0]], 2),
        Eg([[0, 0, 0], [1, 1, 0], [1, 1, 0]], 4),
        Eg([[1, 0, 0], [1, 1, 0], [1, 1, 0]], -1),
        Eg([[0, 1, 1, 0, 0, 0], [0, 1, 0, 1, 1, 0], [0, 1, 1, 0, 1, 0],
            [0, 0, 0, 1, 1, 0], [1, 1, 1, 1, 1, 0], [1, 1, 1, 1, 1, 0]], 14)
    ]
    tester_helper(1091, examples, Solution().shortestPathBinaryMatrix, True)
