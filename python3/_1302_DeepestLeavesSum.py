# Definition for a binary tree node.
from typing import List, Optional, Tuple

from python3.helper import Eg, int_list_to_tree, tester_helper
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from python3.helper_classes import TreeNode


class Solution:
    def deepestLeavesSum(self, root: Optional[TreeNode]) -> int:
        return sum(self.get_deepest(root, 0)[1])

    def get_deepest(self, node: Optional[TreeNode], depth: int) \
            -> Tuple[int, List[int]]:
        if node is None:
            return -1, []
        elif node.left is None and node.right is None:
            return depth, [node.val]
        else:
            left_depth, left_list = self.get_deepest(node.left, depth + 1)
            right_depth, right_list = self.get_deepest(node.right, depth + 1)
            if left_depth == right_depth:
                return left_depth, left_list + right_list
            elif left_depth > right_depth:
                return left_depth, left_list
            else:
                assert right_depth > left_depth
                return right_depth, right_list


def tester():
    examples = [
        Eg(int_list_to_tree(
            [1, 2, 3, 4, 5, None, 6, 7, None, None, None, None, 8], TreeNode),
            15),
        Eg(int_list_to_tree(
            [6, 7, 8, 2, 7, 1, 3, 9, None, 1, 4, None, None, None, 5],
            TreeNode),
            19),
    ]
    tester_helper(1302, examples, Solution().deepestLeavesSum)
