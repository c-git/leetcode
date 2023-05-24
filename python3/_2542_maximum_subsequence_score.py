import math
from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def max_score_helper(self, nums1: List[int], nums2: List[int], k: int, sum_nums1: int, min_num2: int,
                         index: int, answer: List[int]):
        # Source: Beixuan
        assert k >= 0
        if k == 0:
            ans = sum_nums1 * min_num2
            if len(answer) == 0:
                answer.append(ans)
            else:
                answer[0] = max(ans, answer[0])
            return
        for i in range(index, len(nums1)):
            self.max_score_helper(
                nums1, nums2, k - 1, sum_nums1 + nums1[i], min(min_num2, nums2[i]), i + 1, answer)

    def maxScore(self, nums1: List[int], nums2: List[int], k: int) -> int:
        answer = []
        self.max_score_helper(nums1, nums2, k, 0, math.inf, 0, answer)
        return answer[0]


def tester():
    examples = [
        Eg(([1, 3, 3, 2], [2, 1, 3, 4], 3), 12),
        Eg(([4, 2, 3, 1, 1], [7, 5, 10, 9, 6], 1), 30),
    ]
    tester_helper(2542, examples, Solution().maxScore)
