# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from typing import Optional, List

from python3.helper import tester_helper, Eg, int_list_to_tree
from python3.helper_classes import TreeNode


class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        result = []

        if root is None:
            return result

        stack = []
        curr = root
        while len(stack) > 0 or curr is not None:
            # Go to the leftmost
            while curr is not None:
                stack.append(curr)
                curr = curr.left
            curr = stack.pop()

            # Perform action
            result.append(curr.val)

            curr = curr.right

        return result


def tester():
    examples = [
        Eg(int_list_to_tree([1, None, 2, 3]), [1, 3, 2]),
        Eg(int_list_to_tree([]), []),
        Eg(int_list_to_tree([1]), [1]),
    ]
    tester_helper(94, examples, Solution().inorderTraversal)
