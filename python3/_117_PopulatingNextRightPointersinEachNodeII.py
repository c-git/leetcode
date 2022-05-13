# Definition for a Node.
from typing import List, Optional, Union

from python3.helper import Eg, tester_helper


class Node:
    def __init__(
            self, val: int = 0, left: 'Node' = None, right: 'Node' = None,
            next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next

    def __repr__(self):
        return f'{self.val} -> {self.next}'


class Solution:
    def connect(self, root: Node) -> Node:
        def add_children_to_queue(node: Node, queue_: List):
            if node.left is not None:
                queue_.append(node.left)
            if node.right is not None:
                queue_.append(node.right)

        if root is None:
            return root
        queue: List[Node] = [root]
        while len(queue) > 0:
            children_queue: List[Node] = []
            last_child = queue.pop(0)
            add_children_to_queue(last_child, children_queue)
            while len(queue) > 0:
                next_child = queue.pop(0)
                last_child.next = next_child
                add_children_to_queue(next_child, children_queue)
                last_child = next_child
            queue = children_queue
        return root


def convertIn(lst: List[Optional[int]]) -> Optional[Node]:
    def int_to_node(val: Optional[int]) -> Optional[Node]:
        if val is None:
            return None
        else:
            return Node(val)

    root = None if len(lst) == 0 else int_to_node(lst.pop(0))
    queue: List[Optional[Node]] = [root]
    while len(queue) > 0 and len(lst) > 0:
        node = queue.pop(0)
        if node is None:
            # None's children must also be None
            assert lst.pop(0) is None
            assert lst.pop(0) is None
            queue += [None] * 2
        else:
            child = int_to_node(lst.pop(0))
            node.left = child
            queue.append(child)
            child = int_to_node(lst.pop(0))
            node.right = child
            queue.append(child)
    return root


def convertOut(root: Node) -> List[Union[int, str]]:
    root = Solution().connect(root)
    result = []
    while root is not None:
        # Navigate down
        leftmost_child = None
        while True:
            # Navigate across
            if root is None:
                result.append('#')
                break
            if leftmost_child is None:
                leftmost_child = root.left if root.left is not None else \
                    root.right
            result.append(root.val)
            root = root.next
        root = leftmost_child
    return result


def tester():
    examples = [
        Eg(convertIn([1, 2, 3, 4, 5, None, 7]),
           [1, '#', 2, 3, '#', 4, 5, 7, '#']),
        Eg(convertIn([]), []),
    ]
    tester_helper(117, examples, convertOut)
