from python3.helper import Eg, tester_helper


class Solution:
    def countVowelStrings(self, n: int) -> int:
        last_values = {}  # Store values needed to compute next set of values

        # Base Case
        for i in range(1, 6):
            last_values[i] = i

        # General Case
        for size in range(2, n + 1):
            for letter_count in range(2, 5 + 1):
                last_values[letter_count] += last_values[letter_count - 1]
        return last_values[5]


def tester():
    examples = [
        Eg(1, 5),
        Eg(2, 15),
        Eg(33, 66045),
    ]
    tester_helper(1641, examples, Solution().countVowelStrings)
