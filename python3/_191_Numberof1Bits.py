from python3.helper import Eg, tester_helper


class Solution:
    def hammingWeight(self, n: int) -> int:
        result = 0
        while n != 0:
            if n % 2 == 1:
                result += 1
            n //= 2
        return result


def tester():
    examples = [
        Eg(0b00000000000000000000000000001011, 3),
        Eg(0b00000000000000000000000010000000, 1),
        Eg(0b11111111111111111111111111111101, 31)
    ]
    tester_helper(191, examples, Solution().hammingWeight)
