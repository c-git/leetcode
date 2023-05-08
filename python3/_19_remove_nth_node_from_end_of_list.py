# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
from collections import deque
from typing import Optional

from python3._2_AddTwoNumbers import ListNode
from python3.helper import Eg, tester_helper, int_list_to_linked_list


class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        last_n_plus_1 = deque()
        n_plus_1 = n + 1
        node = head
        while node is not None:
            last_n_plus_1.append(node)
            if len(last_n_plus_1) > n_plus_1:
                last_n_plus_1.popleft()
            node = node.next
        assert len(last_n_plus_1) <= n_plus_1
        if len(last_n_plus_1) == n_plus_1:
            # Next must exist because n is min 1 and thus n_plus_1 is min 2
            node_before = last_n_plus_1[-n_plus_1]
            node_before.next = node_before.next.next
        elif len(last_n_plus_1) == n:
            # Head should be removed
            head = head.next
        else:
            return None  # Can only not have a previous if it is the only node
        return head


def tester():
    examples = [
        Eg((int_list_to_linked_list([1, 2, 3, 4, 5]), 2), int_list_to_linked_list([1, 2, 3, 5])),
        Eg((int_list_to_linked_list([1]), 1), int_list_to_linked_list([])),
        Eg((int_list_to_linked_list([1, 2]), 1), int_list_to_linked_list([1])),
        Eg((int_list_to_linked_list([1, 2]), 2), int_list_to_linked_list([2])),
    ]
    tester_helper(19, examples, Solution().removeNthFromEnd)
