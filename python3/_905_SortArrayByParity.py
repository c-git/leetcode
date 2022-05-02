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
