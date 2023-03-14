# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from typing import Optional, List

from python3.helper import Eg, tester_helper, int_list_to_tree, evaluator_any_order_list
from python3.helper_classes import TreeNode


class Solution:
    def findMode(self, root: Optional[TreeNode]) -> List[int]:
        # Used editorial for https://leetcode.com/problems/binary-tree-inorder-traversal/editorial/ to get in order traversal
        assert root is not None, "Constraints in question state minim tree size is 1"

        result = []
        highest_freq = 0
        curr_freq = 0
        last_val = root.val

        stack = []
        curr = root
        res = []
        while curr is not None or len(stack) > 0:
            # Get the leftmost node
            while curr is not None:
                stack.append(curr)
                curr = curr.left
            curr = stack.pop()

            # Handle next value in sequence
            if curr.val == last_val:
                # Same value just increment freq
                curr_freq += 1
            else:
                # New value, check if last was part of mode or a start of a new set of mode values
                if curr_freq > highest_freq:
                    highest_freq = curr_freq
                    result = [last_val]
                elif curr_freq == highest_freq:
                    result.append(last_val)
                curr_freq = 1
                last_val = curr.val

            # prepare for next iteration of loop
            curr = curr.right

        # Check for last value in sequence
        if curr_freq > highest_freq:
            result = [last_val]
        elif curr_freq == highest_freq:
            result.append(last_val)

        return result


def tester():
    examples = [
        Eg(int_list_to_tree([1, None, 2, 2, None]), [2], evaluator_any_order_list),
        Eg(int_list_to_tree([0]), [0], evaluator_any_order_list),
    ]
    tester_helper(501, examples, Solution().findMode)
