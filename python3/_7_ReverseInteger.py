from python3.helper import Eg, tester_helper


class Solution:
    def __init__(self):
        import math
        self.min_val = -math.pow(2, 31)
        self.max_val = math.pow(2, 31) - 1

    def reverse(self, x: int) -> int:
        is_negative = x < 0
        if is_negative:
            x *= -1
        result = 0
        while x > 0:
            result *= 10
            result += x % 10
            x = x // 10
        if is_negative:
            result *= -1
        if self.min_val <= result <= self.max_val:
            return result
        else:
            return 0


def tester():
    examples = [
        Eg(123, 321),
        Eg(-123, -321),
        Eg(120, 21),
        Eg(1534236469, 0)
    ]
    tester_helper(7, examples, Solution().reverse)
