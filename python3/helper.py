from timeit import timeit
from typing import Any, Callable, List, Tuple, Union

from opylib.log import setup_log
from opylib.stopwatch import StopWatch


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
          as in the error  message as an example solution NOTE: Should take 2
          arguments the input as a tuple and the output
          and should return a boolean
        """
        self.input_ = input_ if isinstance(input_, Tuple) else (input_,)
        self.expected = expected
        self.evaluator = evaluator

    @property
    def as_tuple(self):
        return self.input_, self.expected, self.evaluator


def tester_helper(prob_num: int, examples: List[Eg], func: Callable,
                  should_test_timing: bool = False):
    if should_test_timing:
        test_vars = {
            'examples': examples, 'func': func, 'test_func': _tester_body}
        print(timeit('test_func(examples, func)', globals=test_vars))
    else:
        setup_log(only_std_out=True, fmt_std_out='%(message)s')
        sw = StopWatch(f'Problem {prob_num}')
        _tester_body(examples, func)
        sw.end()


def _tester_body(examples: List[Eg], func: Callable):
    for example in examples:
        in_, exp, evaluator = example.as_tuple
        out_ = func(*in_)
        result = exp == out_ if evaluator is None else evaluator(in_, out_)
        assert result, f'\ninp: {in_}\nexp: {exp}\nout: {out_}'
