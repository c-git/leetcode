"""
To store class definitions, so they are not automatically included in helper
"""


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __repr__(self):
        return f'({self.val} L:{self.left} R:{self.right})'
