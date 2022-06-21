from heapq import heappush, heappushpop
from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def furthestBuilding(self, heights: List[int], bricks: int,
                         ladders: int) -> int:
        currBuilding: int = 0
        heap = []

        # Fill heap to match number of ladders
        while currBuilding < len(heights) - 1:
            # LI:
            # - There are more buildings to check
            # - All jumps before currBuilding have been handled
            # - All jumps before currBuilding have been handled 
            # - The largest jumps are stored in the heap
            # - Any jumps not in the heap have been removed from the block count
            diff = heights[currBuilding + 1] - heights[currBuilding]
            if diff <= 0:
                currBuilding += 1
                continue  # just step down go onto next building
            if len(heap) < ladders:
                heappush(heap, diff)
            elif ladders == 0:
                if diff > bricks:
                    return currBuilding
                else:
                    bricks -= diff
            else:
                # Heap full and we have ladders
                smallest = heappushpop(heap, diff)
                if smallest > bricks:
                    return currBuilding
                else:
                    bricks -= smallest
            currBuilding += 1
        return currBuilding


def tester():
    examples = [
        Eg(([4, 2, 7, 6, 9, 14, 12], 5, 1), 4),
        Eg(([4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2), 7),
        Eg(([14, 3, 19, 3], 17, 0), 3),
    ]
    tester_helper(1642, examples, Solution().furthestBuilding)
