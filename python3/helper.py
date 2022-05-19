from collections import deque
from copy import copy
from timeit import timeit
from typing import Any, Callable, List, Optional, Tuple, Union

from opylib.log import log, setup_log
from opylib.stopwatch import StopWatch

from python3.helper_classes import TreeNode


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
                  avg_count=3):
    setup_log(only_std_out=True, fmt_std_out='%(message)s')
    sw = StopWatch(f'Problem {prob_num}')
    if should_test_timing:
        test_vars = {
            'examples': examples, 'func': func, 'test_func': _tester_body}
        vals = []
        for i in range(1, avg_count + 1):
            vals.append(timeit('test_func(examples, func)', globals=test_vars,
                               number=timeit_count))
            log(f'{i} of {avg_count} time: {vals[-1]}')
        log(f'Average: {sum(vals) / len(vals)}')
    else:
        _tester_body(examples, func)
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


def int_list_to_tree(lst: List[Optional[int]], node_cls: Callable = TreeNode):
    """
    Takes a list of integers (or None) and returns a tree (or None if None rec)
    :param lst: List of integers to be converted to a tree
    :param node_cls: The class to use to construct the nodes
    :return: Tree representation of the list passed with nodes created using
      node_cls
    """

    def int_to_node(val: Optional[int]) -> Optional[node_cls]:
        if val is None:
            return None
        else:
            return node_cls(val)

    root = None if len(lst) == 0 else int_to_node(lst.pop(0))
    if root is None:
        return None
    queue: deque[node_cls] = deque()
    queue.append(root)
    while len(queue) > 0 and len(lst) > 0:
        node = queue.popleft()

        # Get left child
        child = int_to_node(lst.pop(0))
        if child is not None:
            node.left = child
            queue.append(child)

        # Get right child
        child = int_to_node(lst.pop(0))
        if child is not None:
            node.right = child
            queue.append(child)
    return root


def _tester_body(examples: List[Eg], func: Callable):
    for example in examples:
        in_, exp, evaluator = example.as_tuple
        out_ = func(*in_)
        result = exp == out_ if evaluator is None else evaluator(in_, out_, exp)
        assert result, f'\ninp: {in_}\nout: {out_}\nexp: {exp}'
