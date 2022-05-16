from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def shortestPathBinaryMatrix(self, grid: List[List[int]]) -> int:
        # Src: Modified from Beixuan's solution
        if grid[0][0] or grid[-1][-1]:
            return -1

        n = len(grid)
        my_grid = [[-1] * n for _ in range(n)]
        my_grid[0][0] = 1
        my_list = [(0, 0)]
        while len(my_list):
            x, y = my_list.pop(0)
            a_min = max(0, x - 1)
            a_max = min(n - 1, x + 1)
            b_min = max(0, y - 1)
            b_max = min(n - 1, y + 1)
            for a in range(a_min, a_max + 1):
                for b in range(b_min, b_max + 1):
                    if not grid[a][b] and my_grid[a][b] == -1:
                        my_grid[a][b] = my_grid[x][y] + 1
                        my_list.append((a, b))
        return my_grid[-1][-1]


def tester():
    examples = [
        Eg([[0, 1], [1, 0]], 2),
        Eg([[0, 0, 0], [1, 1, 0], [1, 1, 0]], 4),
        Eg([[1, 0, 0], [1, 1, 0], [1, 1, 0]], -1),
        Eg([[0, 1, 1, 0, 0, 0], [0, 1, 0, 1, 1, 0], [0, 1, 1, 0, 1, 0],
            [0, 0, 0, 1, 1, 0], [1, 1, 1, 1, 1, 0], [1, 1, 1, 1, 1, 0]], 14)
    ]
    tester_helper(1091, examples, Solution().shortestPathBinaryMatrix, True)
