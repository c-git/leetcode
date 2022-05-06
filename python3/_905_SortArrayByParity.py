from typing import List


class Solution:
    def sortArrayByParity(self, nums: List[int]) -> List[int]:
        front = 0
        back = len(nums) - 1
        while front < back:
            # LI: indices below front are correct and indices above
            # back are correct

            # Find next out of place num at front
            while front < back and nums[front] % 2 == 0:
                front += 1

            # Find next out of place num at back
            while front < back and nums[back] % 2 != 0:
                back -= 1

            # Swap
            nums[front], nums[back] = nums[back], nums[front]
            front += 1
            back -= 1
        return nums


def tester():
    print("905 start")
    examples = [
        ([3, 1, 2, 4], [2, 4, 3, 1]),
        ([0], [0]),
    ]
    solution = Solution()
    for example in examples:
        input_, exp = example
        output_ = solution.sortArrayByParity(input_)
        assert validator(output_), \
            f'\n' \
            f'inp: {input_}\n' \
            f'exp: {exp}\n' \
            f'out: {output_}'
    print("905 complete")


def validator(in_l: List[int]) -> bool:
    """
    Check that no even numbers follow odd numbers
    :param in_l: input list
    :return: True if in_l meets problem constraints
    """
    is_last_even = True
    for val in in_l:
        if val % 2 == 0:
            if not is_last_even:
                return False
        else:
            if is_last_even:
                is_last_even = False
    return True
