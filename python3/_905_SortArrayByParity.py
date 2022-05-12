from typing import List

from python3.helper import Eg, tester_helper


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
    examples = [
        Eg([3, 1, 2, 4], [2, 4, 3, 1], validator),
        Eg([0], [0], validator),
    ]
    tester_helper(905, examples, Solution().sortArrayByParity)


def validator(_, out_: List[int], __) -> bool:
    """
    Check that no even numbers follow odd numbers
    :param _: Input to example but not needed for validation
    :param out_: Output to ve validated
    :param __: Expected output but not needed for validation
    :return: True if out_ meets problem constraints
    """
    is_last_even = True
    for val in out_:
        if val % 2 == 0:
            if not is_last_even:
                return False
        else:
            if is_last_even:
                is_last_even = False
    return True
