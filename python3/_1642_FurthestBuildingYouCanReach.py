from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def furthestBuilding(self, heights: List[int], bricks: int,
                         ladders: int, currBuilding: int = 0) -> int:
        assert currBuilding <= len(heights)  # Not passed end
        if len(heights) - 1 == currBuilding:
            # At last building return index
            return currBuilding
        heightDiff = heights[currBuilding + 1] - heights[currBuilding]
        if heightDiff <= 0:
            # Just move to next building no resources needed
            return self.furthestBuilding(heights, bricks, ladders,
                                         currBuilding + 1)
        if heightDiff > bricks:
            # Not enough bricks
            if ladders > 0:
                return self.furthestBuilding(heights, bricks, ladders - 1,
                                             currBuilding + 1)
            else:
                return currBuilding
        elif ladders == 0:
            # Must use bricks
            return self.furthestBuilding(heights, bricks - heightDiff, ladders,
                                         currBuilding + 1)
        else:
            # Can use ladder or bricks
            return max(
                self.furthestBuilding(heights, bricks - heightDiff, ladders,
                                      currBuilding + 1),
                self.furthestBuilding(heights, bricks, ladders - 1,
                                      currBuilding + 1))


def tester():
    examples = [
        Eg(([4, 2, 7, 6, 9, 14, 12], 5, 1), 4),
        Eg(([4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2), 7),
        Eg(([14, 3, 19, 3], 17, 0), 3),
    ]
    tester_helper(1642, examples, Solution().furthestBuilding)
