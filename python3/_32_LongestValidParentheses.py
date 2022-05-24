# src: Discussion with Beixuan
# NB: Made several incorrect submissions on website trying to code it directly
# there
from collections import deque
from typing import Deque, Tuple

from python3.helper import Eg, tester_helper


class Solution:
    def longestValidParentheses(self, s: str) -> int:
        stack = deque()  # Stores indices of open brackets
        max_ = 0  # Longest set of matches found

        # (Matching index, Match Stack Level)
        matches: Deque[Tuple[int, int]] = deque()

        for i, c in enumerate(s):
            if c == '(':
                stack.append(i)  # Store open bracket index
            else:
                # c == ')'
                if len(stack) > 0:
                    curr = stack.pop()  # Get current matching index
                    while len(matches) > 0 and curr < matches[-1][0]:
                        # Get rid of matches that are not better
                        matches.pop()
                    if len(matches) == 0 or len(stack) > matches[-1][1]:
                        # No better match stored, add this one
                        matches.append((curr, len(stack)))
                    max_ = max(max_, i - matches[-1][0] + 1)
                else:
                    matches.clear()  # All invalid just discard
        return max_


def tester():
    examples = [
        Eg("(()", 2),
        Eg(")()())", 4),
        Eg("", 0),
        Eg("()((())()", 6),
        Eg("()((())())", 10),
    ]
    tester_helper(32, examples, Solution().longestValidParentheses)
