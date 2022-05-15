# Definition for a Node.
from typing import Optional

from python3._117_PopulatingNextRightPointersinEachNodeII import (Node,
                                                                  convertOut)
from python3.helper import Eg, int_list_to_tree, tester_helper

"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = 
    None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""


class Solution:
    def connect(self, root: Optional[Node]) -> Optional[Node]:
        # After looking at many solutions to find understand how a recursive
        # solution would work. Initially thought of BFS like how I did 117.
        # Partially I would never have devised a solutions such as this one
        # because of not incorporating the perfect binary tree assumption

        # Src: https://leetcode.com/problems/populating-next-right-pointers
        # -in-each-node/discuss/2039292/100-time-beaten-O(
        # 1)-Space-recursive-solution
        if root is not None and root.left is not None:
            root.left.next = root.right
            if root.next is not None:
                root.right.next = root.next.left
            self.connect(root.left)
            self.connect(root.right)
        return root


def tester():
    examples = [
        Eg(int_list_to_tree([1, 2, 3, 4, 5, 6, 7], Node),
           [1, '#', 2, 3, '#', 4, 5, 6, 7, '#'], convertOut),
        Eg(int_list_to_tree([], Node), [], convertOut),
    ]
    tester_helper(116, examples, Solution().connect)
