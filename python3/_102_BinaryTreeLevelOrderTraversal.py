from typing import List, Optional

from python3.helper import Eg, int_list_to_tree, tester_helper
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from python3.helper_classes import TreeNode


class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []
        result = []
        level = [root]
        while len(level) > 0:
            level_vals = []
            next_level = []
            for node in level:
                level_vals.append(node.val)
                if node.left is not None:
                    next_level.append(node.left)
                if node.right is not None:
                    next_level.append(node.right)
            result.append(level_vals)
            level = next_level
        return result


def tester():
    examples = [
        Eg(int_list_to_tree([3, 9, 20, None, None, 15, 7]),
           [[3], [9, 20], [15, 7]]),
        Eg(int_list_to_tree([1]), [[1]]),
        Eg(int_list_to_tree([]), [])
    ]
    tester_helper(102, examples, Solution().levelOrder)
