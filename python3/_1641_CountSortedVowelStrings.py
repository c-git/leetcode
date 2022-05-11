from python3.helper import Eg, tester_helper


class Solution:
    def countVowelStrings(self, n: int) -> int:
        vowels = 'aeiou'
        result = 0
        stack = ['']
        while len(stack) > 0:
            prefix = stack.pop()
            for c in vowels:
                if len(prefix) == 0 or prefix[-1] <= c:
                    word = prefix + c
                    if len(word) == n:
                        result += 1
                    else:
                        stack.append(word)
        return result


def tester():
    examples = [
        Eg(1, 5),
        Eg(2, 15),
        Eg(33, 66045),
    ]
    tester_helper(1641, examples, Solution().countVowelStrings)
