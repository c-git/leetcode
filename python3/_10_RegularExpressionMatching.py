from collections import deque
from typing import List, Tuple

from python3.helper import Eg, tester_helper


class Solution:
    def regex_compile(self, pattern: str) -> List[Tuple[str, bool]]:
        """
        Returns a list of tuples where each tuple has a character to be
        matched with
            false if it must be matched exactly one or
            true if it can be 0 to as many as
        """
        result = []
        last_char = None
        for c in pattern:
            if c == '*':
                result.append((last_char, True))
            else:
                if last_char is not None and last_char != '*':
                    result.append((last_char, False))
            last_char = c
        if last_char != '*':
            result.append((last_char, False))
        return result

    def isMatch(self, s: str, p: str) -> bool:
        matcher = self.regex_compile(p)
        possible = deque()
        possible.append((0,
                         0))  # first value is position in str, second is
        # position in matcher
        while len(possible) > 0:
            i_s, i_m = possible.pop()
            s_v, m_v = s[i_s], matcher[i_m]
            if not m_v[1]:
                # Excatly once
                if s_v != m_v[0] and m_v[0] != '.':
                    continue  # This branch cannot work discard
                if i_s < len(s) - 1:
                    # String has more char to be matched
                    if i_m < len(matcher) - 1:
                        # Has more regex add next part to be matched
                        possible.append((i_s + 1, i_m + 1))
                    else:
                        # no more regex
                        continue
                else:
                    # No more string
                    dead_end = False
                    for i in range(i_m + 1, len(matcher)):
                        # Check any remaining regex to ensure it can match 0
                        # times
                        if not matcher[i][1]:
                            dead_end = True
                            break
                    if not dead_end:
                        return True
            else:
                # 0 or more times
                has_more_regex = i_m + 1 < len(matcher)
                while True:
                    if has_more_regex:
                        # We could have moved onto the next matcher value
                        possible.append((i_s, i_m + 1))
                    if s_v != m_v[0] and m_v[0] != '.':
                        break
                    else:
                        i_s += 1
                        if i_s < len(s):
                            s_v = s[i_s]
                        else:
                            # No more string
                            dead_end = False
                            for i in range(i_m, len(matcher)):
                                # Check any remaining regex to ensure it can
                                # match 0 times
                                if not matcher[i][1]:
                                    dead_end = True
                                    break
                            if not dead_end:
                                return True
                            else:
                                break
        return False


def tester():
    examples = [
        Eg(("aa", "a"), False),
        Eg(("aa", "a*"), True),
        Eg(("ab", ".*"), True),
    ]
    tester_helper(10, examples, Solution().isMatch)
