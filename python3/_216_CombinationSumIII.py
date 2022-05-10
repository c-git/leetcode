from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def solve(self, k, n, low_num) -> List[List[int]]:
        if n <= 0:
            return []
        if low_num > 9:
            return []
        if k == 1:
            if low_num <= n <= 9:
                return [[n]]
            else:
                return []
        result = []
        for num in range(low_num, 10):
            candidate = [num]
            friend_solutions = self.solve(k - 1, n - num, num + 1)
            if len(friend_solutions) == 0:
                break  # Already no solutions left no point for more iterations
            for sol in friend_solutions:
                solution = candidate + sol
                result.append(solution)
        return result

    def combinationSum3(self, k: int, n: int) -> List[List[int]]:
        return self.solve(k, n, 1)


def tester():
    examples = [
        Eg((3, 7), [[1, 2, 4]]),
        Eg((3, 9), [[1, 2, 6], [1, 3, 5], [2, 3, 4]]),
        Eg((4, 1), []),
        Eg((3, 15),
           [[1, 5, 9], [1, 6, 8], [2, 4, 9], [2, 5, 8], [2, 6, 7], [3, 4, 8],
            [3, 5, 7], [4, 5, 6]]),
    ]
    tester_helper(216, examples, Solution().combinationSum3, True)
