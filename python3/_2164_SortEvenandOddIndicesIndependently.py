from typing import List


class Solution:
    def sortEvenOdd(self, nums: List[int]) -> List[int]:
        even = []
        odd = []
        for i, num in enumerate(nums):
            if i % 2 == 0:
                even.append(num)
            else:
                odd.append(num)
        even.sort()
        odd.sort()
        odd.reverse()
        result = []
        for e, o in zip(even, odd):
            result.append(e)
            result.append(o)
        if len(even) > len(odd):
            result.append(even[-1])
        elif len(odd) > len(even):
            result.append(odd[-1])
        return result
