import numpy as np

from python3.helper import Eg, tester_helper


class Solution:
    def countVowelStrings(self, n: int) -> int:
        table = np.zeros((n + 1, 6))

        # Base Case
        for i in range(6):
            table[1, i] = i

        # General Case
        for size in range(2, n + 1):
            for letter_count in range(1, 5 + 1):
                first_char_fixed_as_curr = table[size - 1, letter_count]
                without_curr_char = table[size, letter_count - 1]
                table[size, letter_count] = (first_char_fixed_as_curr +
                                             without_curr_char)
        return int(table[n, 5])


def tester():
    examples = [
        Eg(1, 5),
        Eg(2, 15),
        Eg(33, 66045),
    ]
    tester_helper(1641, examples, Solution().countVowelStrings)
