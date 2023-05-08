from typing import Optional

from python3.helper import Eg, int_list_to_linked_list, tester_helper
from python3.helper_classes import ListNode


class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return None
        if head.next is None:
            return head
        odd = head
        even = head.next
        curr = even.next
        is_odd = True
        even_head = even
        while curr is not None:
            # LI
            # - All nodes before curr are already in the even list or odd list
            # - head points to the odd head
            # - even_head points to the even head
            # - last odd node is pointed to by odd
            # - last even node is pointed to by even
            # - is_odd matches parity of current node index
            if is_odd:
                odd.next = curr
                odd = curr
            else:
                even.next = curr
                even = curr
            curr = curr.next
            is_odd = not is_odd
        even.next = None
        odd.next = even_head
        return head


def tester():
    build = int_list_to_linked_list
    examples = [
        Eg(build([1, 2, 3, 4, 5]), build([1, 3, 5, 2, 4])),
        Eg(build([2, 1, 3, 5, 6, 4, 7]), build([2, 3, 6, 7, 1, 5, 4])),
        Eg(build([2, ]), build([2, ])),
        Eg(build([2, 1]), build([2, 1])),
        Eg(build([2, 1, 3]), build([2, 3, 1])),
        Eg(build([2, 1, 3, 4]), build([2, 3, 1, 4])),
        Eg(build([]), build([])),

    ]
    tester_helper(328, examples, Solution().oddEvenList)
