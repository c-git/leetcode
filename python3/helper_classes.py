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


class ListNode:
    def __init__(self, x: int, next=None):
        self.val = x
        self.next = next

    def __repr__(self):
        return f'{self.val} -> {self.next}'

    def __eq__(self, other):
        if isinstance(other, ListNode):
            return self.val == other.val and self.next == other.next
        return False
