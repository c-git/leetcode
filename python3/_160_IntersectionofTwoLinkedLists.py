from typing import List, Optional

from python3.helper import Eg, int_list_to_linked_list, tester_helper
from python3.helper_classes import ListNode


class Solution:
    def getIntersectionNode(
            self, headA: ListNode, headB: ListNode) -> Optional[ListNode]:
        seen = set()

        # Load all nodes from A into see
        node = headA
        while node is not None:
            seen.add(node)
            node = node.next

        # Check if we've seen any of the nodes in B before
        node = headB
        while node is not None:
            if node in seen:
                return node
            node = node.next

        return None


def input_builder(listA: List[int], listB: List[int], skipA: int, skipB: int):
    assert len(listA) >= skipA
    assert len(listB) >= skipB
    headA = int_list_to_linked_list(listA)

    if skipB == 0:
        return headA, headA

    headB = int_list_to_linked_list(listB)

    count = 0
    nodeA = headA
    while True:
        if count >= skipA:
            break
        count += 1
        nodeA = nodeA.next

    count = 0
    nodeB = headB
    while True:
        if count >= skipB - 1:
            nodeB.next = nodeA
            break
        count += 1
        nodeB = nodeB.next

    return headA, headB


def evaluator(_, out_, exp) -> bool:
    return (exp == 0) if out_ is None else (out_.val == exp)


def tester():
    ev = evaluator
    examples = [
        Eg(input_builder([4, 1, 8, 4, 5], [5, 6, 1, 8, 4, 5], 2, 3), 8, ev),
        Eg(input_builder([1, 9, 1, 2, 4], [3, 2, 4], 3, 1), 2, ev),
        Eg(input_builder([2, 6, 4], [1, 5], 3, 2), 0, ev),
    ]
    tester_helper(160, examples, Solution().getIntersectionNode)
