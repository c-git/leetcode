from typing import List, Optional, Tuple


# Definition for singly-linked list.
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


def convertToListNodes(l_in: List[int]) -> ListNode:
    head = ListNode(l_in[0])
    last = head
    for i in range(1, len(l_in)):
        next_ = ListNode(l_in[i])
        last.next = next_
        last = next_
    return head


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
