from python3.helper import Eg, tester_helper


class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        substring = ''
        chars = {}
        result = 0
        start = 0
        for i, c in enumerate(s):
            pos = chars.get(c)
            if pos is not None:
                if len(substring) > result:
                    result = len(substring)
                change = pos - start + 1
                for rem in substring[:change - 1]:
                    chars.pop(rem)
                substring = substring[change:]
                start += change
            substring += c
            chars[c] = i
        if len(substring) > result:
            result = len(substring)
        return result


def tester():
    examples = [
        Eg("abcabcbb", 3),
        Eg("bbbbb", 1),
        Eg("pwwkew", 3),
    ]
    tester_helper(216, examples, Solution().lengthOfLongestSubstring)
