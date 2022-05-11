from python3.helper import Eg, tester_helper


class Solution:
    def solve(self, n, letter_count) -> int:
        result = 0
        if n == 1:
            return letter_count
        for i in range(letter_count):
            result += self.solve(n - 1, letter_count - i)
        return result

    def countVowelStrings(self, n: int) -> int:
        vowels = 5
        return self.solve(n, 5)


def tester():
    examples = [
        Eg(1, 5),
        Eg(2, 15),
        Eg(33, 66045),
    ]
    tester_helper(1641, examples, Solution().countVowelStrings)
