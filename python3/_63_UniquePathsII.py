from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    # DFS was much too slow and possibly wrong.
    # Using DP solution provided by LeetCode
    # Src: LeetCode
    def uniquePathsWithObstacles(self, obstacleGrid: List[List[int]]) -> int:
        # obstacleGrid = deepcopy(obstacleGrid)
        if obstacleGrid[0][0] == 1 or obstacleGrid[-1][-1] == 1:
            return 0

        # Base cases
        val = 1
        for i in range(len(obstacleGrid)):
            if obstacleGrid[i][0] == 1:
                val = 0
            obstacleGrid[i][0] = val
        val = 1
        for i in range(1, len(obstacleGrid[0])):
            if obstacleGrid[0][i] == 1:
                val = 0
            obstacleGrid[0][i] = val

        # General Case
        for row in range(1, len(obstacleGrid)):
            for col in range(1, len(obstacleGrid[0])):
                obstacleGrid[row][col] = (
                    0
                    if obstacleGrid[row][col] == 1 else
                    obstacleGrid[row - 1][col] + obstacleGrid[row][col - 1]
                )

        return obstacleGrid[-1][-1]


def tester():
    examples = [
        Eg([[0, 0, 0], [0, 1, 0], [0, 0, 0]], 2),
        Eg([[0, 1], [0, 0]], 1),
        Eg([[0]], 1),
        Eg([[0, 0], [1, 1], [0, 0]], 0)
    ]
    tester_helper(63, examples, Solution().uniquePathsWithObstacles)
