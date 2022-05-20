from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def __init__(self):
        self.m = 0
        self.n = 0
        self.grid = []

    def uniquePathsWithObstacles(self, obstacleGrid: List[List[int]]) -> int:
        self.grid = obstacleGrid
        self.m = len(self.grid)
        self.n = len(self.grid[0])
        return self.dfs(0, 0)

    def dfs(self, row: int, col: int) -> int:
        if row >= self.m or col >= self.n:
            return 0

        if self.grid[row][col] == 1:
            return 0

        if row == self.m - 1 and col == self.n - 1:
            return 1  # Reached target

        return self.dfs(row + 1, col) + self.dfs(row, col + 1)


def tester():
    examples = [
        Eg([[0, 0, 0], [0, 1, 0], [0, 0, 0]], 2),
        Eg([[0, 1], [0, 0]], 1)
    ]
    tester_helper(63, examples, Solution().uniquePathsWithObstacles)
