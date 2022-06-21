from python3.helper import Eg, tester_helper


class Solution:
    def maxValue(self, n: str, x: int) -> str:
        x = str(x)
        is_negative = n[0] == '-'
        i = 1 if is_negative else 0

        while i < len(n):
            if is_negative:
                if x <= n[i]:
                    break
            else:
                # Positive number
                if x >= n[i]:
                    break
            i += 1
        return n[:i] + x + n[i:]


def tester():
    examples = [
        Eg(("99", 9), "999"),
        Eg(("-13", 2), "-123"),
    ]
    tester_helper(1881, examples, Solution().maxValue)
