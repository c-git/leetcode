# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from typing import Optional, Union

from python3.helper import tester_helper, Eg, int_list_to_tree
from python3.helper_classes import TreeNode


class Solution:
    def not_bal_or_max_height(self, root: Optional[TreeNode]) -> Union[bool, int]:
        """
        Checks the height of the tree at `root`. If not balanced returns False. Otherwise, returns larger of children
            heights. Note: never returns True.
        :param root: The root of the tree to check the height of
        """
        if root is None:
            return 0
        left_height = self.not_bal_or_max_height(root.left)
        if type(left_height) == bool and not left_height:
            return False
        right_height = self.not_bal_or_max_height(root.right)
        if type(right_height) == bool and not right_height:
            return False
        assert type(left_height) == int and type(right_height) == int
        if abs(left_height - right_height) > 1:
            return False  # Not balanced
        else:
            return max(left_height, right_height) + 1

    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        result = self.not_bal_or_max_height(root)
        return type(result) == int


def tester():
    examples = [
        Eg(int_list_to_tree([3, 9, 20, None, None, 15, 7]), True),
        Eg(int_list_to_tree([1, 2, 2, 3, 3, None, None, 4, 4]), False),
        Eg(int_list_to_tree([]), True),
    ]
    tester_helper(110, examples, Solution().isBalanced)
