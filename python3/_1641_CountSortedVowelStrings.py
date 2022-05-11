from python3.helper import Eg, tester_helper


class Solution:
    def countVowelStrings(self, n: int) -> int:
        stack = [(n, 5)]
        result = 0
        while len(stack) > 0:
            size, letter_count = stack.pop()
            if size == 1:
                result += letter_count
            else:
                for i in range(letter_count):
                    stack.append((size - 1, letter_count - i))
        return result


def tester():
    examples = [
        Eg(1, 5),
        Eg(2, 15),
        Eg(33, 66045),
    ]
    tester_helper(1641, examples, Solution().countVowelStrings)
