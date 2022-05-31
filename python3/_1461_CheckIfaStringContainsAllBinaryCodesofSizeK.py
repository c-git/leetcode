from python3.helper import Eg, tester_helper


class Solution:
    def hasAllCodes(self, s: str, k: int) -> bool:
        if len(s) < 2 * k:
            return False
        seen = set()
        num_strs = 2 ** k
        for i in range(len(s) - k + 1):
            sub_str = s[i:i + k]
            seen.add(sub_str)
            if len(seen) >= num_strs:
                return True
        return False


def tester():
    examples = [
        Eg(("00110110", 2), True),
        Eg(("0110", 1), True),
        Eg(("0110", 2), False)
    ]
    tester_helper(1461, examples, Solution().hasAllCodes)
