from typing import List, Tuple

from python3.helper import Eg, tester_helper


class Solution:
    def solve(self, k, n, low_num) -> Tuple[List[List[int]], bool]:
        """
        Solves the instance passed recursively
        :param k: the number of digits to be added to the solution
        :param n: the target number to solve for
        :param low_num: smallest number that can be used as part of the solution
        :return: (Solution, should_keep_trying) - The possible solutions and
          if no solutions were found should_keep_trying says if it was
          because the target was too small, this allows the caller to know
          that they should continue to look for solutions with bigger
          starting values (meaning a smaller n passed to later instances)
        """
        if k == 1:
            # Base case, return trivial solution
            if low_num <= n < 10:
                return [[n]], False
            elif n > 9:
                # Target above possible range, but might be possible with for n
                return [], True
            else:
                # n - less than low_num, smaller n would also not be possible
                return [], False
        if n <= 0:
            # Target was too small we over shot the target
            return [], False
        if low_num > 9:
            # The digit that would have gone in this position has gotten too big
            return [], False
        if sum(range(9 - k, 10, )) < n:
            # Max value achievable is too small no solution possible,
            # but might be possible with for n
            return [], True
        if sum(range(low_num, low_num + k)) > n:
            # Min value achievable is already larger than n
            return [], False
        result = []
        for num in range(low_num, 10):
            candidate = [num]
            friend_sol, should_keep_try = self.solve(k - 1, n - num, num + 1)
            if len(friend_sol) == 0 and not should_keep_try:
                break  # Already no solutions left no point for more iterations
            for sol in friend_sol:
                solution = candidate + sol
                result.append(solution)
        return result, False

    def combinationSum3(self, k: int, n: int) -> List[List[int]]:
        return self.solve(k, n, 1)[0]


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
