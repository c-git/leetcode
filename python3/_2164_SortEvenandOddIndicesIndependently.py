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


def tester():
    print("2164 start")
    examples = [
        ([4, 1, 2, 3], [2, 3, 4, 1]),
        ([2, 1], [2, 1]),
    ]
    solution = Solution()
    for example in examples:
        input_, exp = example
        output_ = solution.sortEvenOdd(input_)
        assert exp == output_, \
            f'\n' \
            f'inp: {input_}\n' \
            f'exp: {exp}\n' \
            f'out: {output_}'
    print("2164 complete")
