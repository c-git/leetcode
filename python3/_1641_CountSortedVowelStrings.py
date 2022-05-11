from python3.helper import Eg, tester_helper


class Solution:
    def solve(self, n, letters) -> int:
        result = 0
        if n == 1:
            return len(letters)
        for i in range(len(letters)):
            result += self.solve(n - 1, letters[i:])
        return result

    def countVowelStrings(self, n: int) -> int:
        vowels = 'aeiou'
        return self.solve(n, vowels)


def tester():
    examples = [
        Eg(1, 5),
        Eg(2, 15),
        Eg(33, 66045),
    ]
    tester_helper(1641, examples, Solution().countVowelStrings)
