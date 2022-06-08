from python3.helper import Eg, tester_helper


class Solution:
    def removePalindromeSub(self, s: str) -> int:
        # src: After looking at the picture in this solution it became
        # obvious that I misunderstood the question
        # https://leetcode.com/problems/remove-palindromic-subsequences
        # /discuss/2124240/One-Major-Observation-or-JAVA-Explanation
        if len(s) == 0:
            return 0
        if self.isPalindrome(s):
            return 1
        else:
            # if not palindrome then in one step we can  remove all a's then
            # next step remove all b's. You are only required to remove
            # palindromic subsequences
            return 2

    def isPalindrome(self, s: str) -> bool:
        for i in range(len(s) // 2 + 1):
            if s[i] != s[-i - 1]:
                return False
        return True


def tester():
    examples = [
        Eg('ababa', 1),
        Eg("abb", 2),
        Eg("baabb", 2),
        Eg("babbbbbbba", 2),
        Eg("bbaabb", 1),
    ]
    tester_helper(1332, examples, Solution().removePalindromeSub)
