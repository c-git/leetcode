from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        seen = {}
        for i, num in enumerate(nums):
            diff = target - num
            if seen.get(diff) is not None:
                return [seen[diff], i]
            else:
                seen[num] = i
        return []  # Not found


def tester():
    print("1 start")
    examples = [
        (([2, 7, 11, 15], 9), [0, 1]),
        (([3, 2, 4], 6), [1, 2]),
        (([3, 3], 6), [0, 1]),
    ]
    solution = Solution()
    for example in examples:
        input_, exp = example
        output_ = solution.twoSum(*input_)
        assert exp == output_, f'\ninp: {input_}\nexp: {exp}\nout: {output_}'
    print("1 complete")
