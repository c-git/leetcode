from typing import Optional, Tuple

# Definition for singly-linked list.
from python3.helper import Eg, int_list_to_linked_list, tester_helper


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __eq__(self, other):
        if other is None:
            return False
        return self.val == other.val and self.next == other.next

    def __repr__(self):
        return f'{self.val} -> {self.next}'


class Solution:
    def sum(self, l1: Optional[ListNode], l2: Optional[ListNode], carry: int) \
            -> Tuple[ListNode, int]:
        v1 = 0 if l1 is None else l1.val
        v2 = 0 if l2 is None else l2.val
        sum_ = v1 + v2 + carry
        digit = sum_ % 10
        return ListNode(digit), sum_ // 10

    def addTwoNumbers(
            self, l1: Optional[ListNode], l2: Optional[ListNode]) -> \
            Optional[ListNode]:
        head, carry = self.sum(l1, l2, 0)
        last = head
        l1 = l1.next
        l2 = l2.next
        while l1 is not None or l2 is not None or carry > 0:
            next_, carry = self.sum(l1, l2, carry)
            last.next = next_
            last = next_
            if l1 is not None:
                l1 = l1.next
            if l2 is not None:
                l2 = l2.next
        return head


def tester():
    cvt = int_list_to_linked_list
    ln = ListNode
    examples = [
        Eg((cvt([2, 4, 3], ln), cvt([5, 6, 4], ln)), cvt([7, 0, 8], ln)),
        Eg((cvt([0], ln), cvt([0], ln)), cvt([0], ln)),
        Eg((cvt([9, 9, 9, 9, 9, 9, 9], ln), cvt([9, 9, 9, 9], ln)),
           cvt([8, 9, 9, 9, 0, 0, 0, 1], ln)),
    ]
    tester_helper(2, examples, Solution().addTwoNumbers)
