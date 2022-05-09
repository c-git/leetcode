from collections.abc import Iterable
from typing import List, Tuple

from python3.helper import Eg, tester_helper


class Solution:
    _mapping = None

    @classmethod
    @property
    def mapping(cls):
        if cls._mapping is None:
            cls._mapping = {}
            char_ord = 97  # a
            for digit in range(2, 10):
                chars = []
                cls._mapping[digit] = chars
                char_count = 4 if digit == 7 or digit == 9 else 3
                for _ in range(char_count):
                    chars.append(chr(char_ord))
                    char_ord += 1
            assert char_ord == 123  # after z
        return cls._mapping

    def letterCombinations(self, digits: str) -> List[str]:
        result = []
        chars = [self.mapping[int(digit)] for digit in digits]
        stack: List[Tuple[Iterable, str]] = [] if len(digits) == 0 else \
            [((iter(chars[0])), '')]
        while len(stack) > 0:
            iterator, prefix = stack.pop()
            if len(digits) - 1 == len(prefix):
                # On last digit
                for suffix in iterator:
                    result.append(prefix + suffix)
            else:
                try:
                    new_prefix = prefix + next(iterator)
                    stack.append((iterator, prefix))
                    stack.append((iter(chars[len(new_prefix)]), new_prefix))
                except StopIteration:
                    pass  # Do nothing this digit is completed
        return result


def tester():
    examples = [
        Eg("23", ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]),
        Eg("234",
           ["adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi",
            "bdg", "bdh", "bdi", "beg", "beh", "bei", "bfg", "bfh", "bfi",
            "cdg", "cdh", "cdi", "ceg", "ceh", "cei", "cfg", "cfh", "cfi", ]),
        Eg("", []),
        Eg("2", ["a", "b", "c"]),
        Eg("7", ["p", "q", "r", "s"]),
    ]
    tester_helper(17, examples, Solution().letterCombinations)
