# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from typing import List, Optional

from python3.helper import Eg, tester_helper, evaluator_height_balanced
from python3.helper_classes import TreeNode


class Solution:
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]:
        # Base case
        if len(nums) == 0:
            return None

        # Recursive case
        root_index = len(nums) // 2
        root = TreeNode(nums[root_index])
        root.left = self.sortedArrayToBST(nums[:root_index])
        root.right = self.sortedArrayToBST(nums[root_index + 1:])
        return root


def tester():
    examples = [
        Eg([-10, -3, 0, 5, 9], None, evaluator_height_balanced),
        Eg([1, 3], None, evaluator_height_balanced),
    ]
    tester_helper(108, examples, Solution().sortedArrayToBST)
