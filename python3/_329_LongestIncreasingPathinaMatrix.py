from dataclasses import dataclass
from enum import Enum
from typing import List, Optional, Tuple

from python3.helper import Eg, tester_helper


class Direction(Enum):
    UP = 'U'
    DOWN = 'D'
    LEFT = 'L'
    RIGHT = 'R'


@dataclass
class TrackData:
    count: int = 1
    next: Optional[Direction] = None
    is_finalized: bool = False

    def update(self, count: int, next_: Direction):
        assert not self.is_finalized
        if count > self.count:
            self.count = count
            self.next = next_
        elif self.next is None:
            self.next = next_

    def finalize(self):
        """
        Called at end to update value to include count for node. No further
        updates expected after this is done
        :return:
        """
        self.count += 1
        self.is_finalized = True

    def __repr__(self):
        return f'{self.count} {"-" if self.next is None else self.next.value}'


class Solution:
    def __init__(self):
        self.m = 1
        self.n = 1
        self.matrix = []
        self.track = []

    def longestIncreasingPath(self, matrix: List[List[int]]) -> int:
        # Set values here to simplify code instead of passing to each
        # recursive call
        self.m = len(matrix)
        self.n = len(matrix[0])
        self.matrix = matrix

        # Table of how cells are related
        self.track: List[List[Optional[TrackData]]] = \
            [[None] * self.n for __ in range(self.m)]

        # Build ordered list of cell values
        cells: List[Tuple[int, int, int]] = []  # (value, row, col)
        for row in range(self.m):  # row
            for col in range(self.n):  # col
                cells.append((matrix[row][col], row, col))

        cells.sort()
        longest = -1

        # Do DFS on starting on smallest cell values
        for _, row, col in cells:
            if self.track[row][col] is None:
                longest = max(longest, self.dfs(row, col))

        return longest

    def dfs(self, row: int, col: int) -> int:
        """
        Return max depth reached found
        :param row: Valid row index
        :param col: Valid column index
        :return:
        """
        # Note: it is impossible to get a cycle because only increasing
        # values are allowed
        val = self.matrix[row][col]  # TODO Remove test code
        if self.track[row][col] is not None:
            return self.track[row][col].count
        tracking = TrackData(0)

        # Check Up
        if row > 0 and self.matrix[row][col] < self.matrix[row - 1][col]:
            tracking.update(self.dfs(row - 1, col), Direction.UP)

        # Check Down
        if row < self.m - 1 \
                and self.matrix[row][col] < self.matrix[row + 1][col]:
            tracking.update(self.dfs(row + 1, col), Direction.DOWN)

        # Check Left
        if col > 0 and self.matrix[row][col] < self.matrix[row][col - 1]:
            tracking.update(self.dfs(row, col - 1), Direction.LEFT)

        # Check Right
        if col < self.n - 1 \
                and self.matrix[row][col] < self.matrix[row][col + 1]:
            tracking.update(self.dfs(row, col + 1), Direction.RIGHT)

        # Update tracking
        assert self.track[row][col] is None
        tracking.finalize()
        self.track[row][col] = tracking

        return tracking.count


def tester():
    examples = [
        Eg([[9, 9, 4], [6, 6, 8], [2, 1, 1]], 4),
        Eg([[3, 4, 5], [3, 2, 6], [2, 2, 1]], 4),
        Eg([[7, 8, 9], [9, 7, 6], [7, 2, 3]], 6),
        Eg([
            [5, 4, 2, 5, 6, 7, 2],
            [5, 6, 7, 4, 3, 8, 8],
            [4, 9, 8, 3, 2, 9, 4],
            [3, 2, 1, 2, 1, 5, 3],
            [2, 9, 1, 1, 3, 1, 2],
            [2, 0, 9, 8, 7, 6, 1],
        ], 9),
    ]
    tester_helper(329, examples, Solution().longestIncreasingPath)
