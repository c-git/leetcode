from collections import deque
from copy import copy, deepcopy
from timeit import timeit
from typing import Any, Callable, List, Optional, Tuple, Union

from opylib.log import log, setup_log
from opylib.stopwatch import StopWatch

from python3.helper_classes import ListNode, TreeNode


class Eg:  # Example
    def __init__(self, input_: Union[Tuple, Any], expected: Any,
                 evaluator: Callable = None):
        """
        Creates an example test case
        :param input_: Input to test case. (NOTE: Enclosed in Tuple if there
        are multiple arguments)
        :param expected: Expected answer for test case
        :param evaluator: If not None then it is used to test the output
          instead of comparing it to "expected" but expected is still displayed
          as in the error  message as an example solution
          NOTE: signature evaluator(in_:Tuple[Any], out_, exp) -> bool
          where
           in_ is the original input in a tuple (unless it was already a tuple
           out_ is the result from running your solution
           exp is the provided solution from LeetCode
        """
        self.input_ = input_ if isinstance(input_, Tuple) else (input_,)
        self.expected = expected
        self.evaluator = evaluator

    @property
    def as_tuple(self):
        return self.input_, self.expected, self.evaluator


def tester_helper(prob_num: int, examples: List[Eg], func: Callable,
                  should_test_timing: bool = False, timeit_count=100000,
                  avg_count=3, copy_input=False):
    setup_log(only_std_out=True, fmt_std_out='%(message)s')
    sw = StopWatch(f'Problem {prob_num}')
    if should_test_timing:
        test_vars = {
            'examples': examples, 'func': func, 'test_func': _tester_body,
            'copy_input': copy_input}
        vals = []
        for i in range(1, avg_count + 1):
            vals.append(timeit('test_func(examples, func, copy_input)',
                               globals=test_vars, number=timeit_count))
            log(f'{i} of {avg_count} time: {vals[-1]}')
        log(f'Average: {sum(vals) / len(vals)}')
    else:
        _tester_body(examples, func, copy_input)
    sw.end()


def evaluator_any_order_list(_, out_: List[Any], exp: List[Any]) -> bool:
    if len(out_) != len(exp):
        return False
    out_ = copy(out_)

    for i in range(len(exp)):
        target = exp[i]
        for j, val in enumerate(out_):
            if val == target:
                out_.pop(j)
                break
        else:
            # Not found in out_
            return False
    assert len(out_) == 0
    return True


def evaluator_sort_to_compare_two_deep(
        _, out_: List[List[Any]], exp: List[List[Any]]) -> bool:
    # Sort inner objects
    for x in out_:
        x.sort()

    out_.sort()  # Sort outer loop

    # Compare
    return out_ == exp


def evaluator_height_balanced(
        _in, out_: Optional[TreeNode], _exp) -> bool:
    # Wrap solution to one of the problems
    from python3._110_balanced_binary_tree import Solution
    return Solution().isBalanced(out_)


def int_list_to_tree(lst: List[Optional[int]], node_cls: Callable = TreeNode):
    """
    Takes a list of integers (or None) and returns a tree (or None if None rec)
    :param lst: List of integers to be converted to a tree
    :param node_cls: The class to use to construct the nodes
    :return: Tree representation of the list passed with nodes created using
      node_cls
    """
    # Changed based on https://leetcode.com/problems/recover-binary-search-tree/solutions/32539/Tree-Deserializer-and-Visualizer-for-Python/

    def int_to_node(val: Optional[int]) -> Optional[node_cls]:
        if val is None:
            return None
        else:
            return node_cls(val)

    root = None if len(lst) == 0 else int_to_node(lst.pop(0))
    if root is None:
        return None
    nodes = [None if val == None else int_to_node(val) for val in lst]
    kids = nodes[::-1]
    root = kids.pop()
    for node in nodes:
        if node:
            if kids:
                node.left = kids.pop()
            if kids:
                node.right = kids.pop()
    return root


def int_list_to_linked_list(lst: List[int], node_cls: Callable = ListNode):
    """
    Takes a list of integers and returns a linked list
    :param lst: List of integers to be converted to a linked list
    :param node_cls: The class to use to construct the nodes
    :return: Linked list of the list passed with nodes created using node_cls
    """
    if len(lst) == 0:
        return None

    head = node_cls(lst[0])
    last = head
    for i in range(1, len(lst)):
        temp = node_cls(lst[i])
        last.next = temp
        last = temp
    return head


def _tester_body(examples: List[Eg], func: Callable, copy_input: bool):
    for example in examples:
        in_, exp, evaluator = example.as_tuple

        # For assert if input is mutilated by caller so that errors are still
        # identifiable
        in_dup = in_ if not copy_input else deepcopy(in_)

        out_ = func(*in_dup)
        result = exp == out_ if evaluator is None else evaluator(
            in_, out_, exp)
        assert result, f'\ninp: {in_}\nout: {out_}\nexp: {exp}'
