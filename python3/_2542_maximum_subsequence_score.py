from queue import PriorityQueue
from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def maxScore(self, nums1: List[int], nums2: List[int], k: int) -> int:
        # Solution from sak96

        # sort by second array
        pairs = sorted(zip(nums1, nums2), key=lambda x: -x[1])

        # get answer first k elements
        q = PriorityQueue()
        sum_ = 0
        for x, y in pairs[:k]:
            q.put(x)
            sum_ += x
        ans = pairs[k - 1][1] * sum_

        sum_ -= q.get()
        for (x, y) in pairs[k:]:
            ans = max(ans, (sum_ + x) * y)
            q.put(x)
            sum_ += x - q.get()

        return ans


def tester():
    examples = [
        Eg(([1, 3, 3, 2], [2, 1, 3, 4], 3), 12),
        Eg(([4, 2, 3, 1, 1], [7, 5, 10, 9, 6], 1), 30),
        Eg((
            [93, 463, 179, 2488, 619, 2006, 1561, 137, 53, 1765, 2304, 1459, 1768, 450, 1938, 2054, 466, 331, 670, 1830,
             1550, 1534, 2164, 1280, 2277, 2312, 1509, 867, 2223, 1482, 2379, 1032, 359, 1746, 966, 232, 67, 1203, 2474,
             944, 1740, 1775, 1799, 1156, 1982, 1416, 511, 1167, 1334, 2344],
            [345, 229, 976, 2086, 567, 726, 1640, 2451, 1829, 77, 1631, 306, 2032, 2497, 551, 2005, 2009, 1855, 1685,
             729, 2498, 2204, 588, 474, 693, 30, 2051, 1126, 1293, 1378, 1693, 1995, 2188, 1284, 1414, 1618, 2005, 1005,
             1890, 30, 895, 155, 526, 682, 2454, 278, 999, 1417, 1682, 995],
            42),
            26653494),
    ]
    tester_helper(2542, examples, Solution().maxScore)
