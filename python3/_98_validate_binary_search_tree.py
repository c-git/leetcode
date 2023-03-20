# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from typing import Optional

from python3.helper import tester_helper, Eg, int_list_to_tree
from python3.helper_classes import TreeNode


class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        last_value = -2 ** 31 - 1  # Less than min specified in question to meet loop invariant on first iteration
        stack = []
        curr = root
        while curr is not None or len(stack) > 0:
            # Get the leftmost node
            while curr is not None:
                stack.append(curr)
                curr = curr.left
            curr = stack.pop()

            # Equal also not allowed based on question
            if curr.val <= last_value:
                return False
            else:
                last_value = curr.val

            # prepare for next iteration of loop
            curr = curr.right

        return True


def tester():
    examples = [
        Eg(int_list_to_tree([2, 1, 3]),
           True),
        Eg(int_list_to_tree([5, 1, 4, None, None, 3, 6]), False),
        Eg(int_list_to_tree([5, 4, 6, None, None, 3, 7]), False),
    ]
    tester_helper(98, examples, Solution().isValidBST)
