class Solution:
    def reverse(self, x: int) -> int:
        is_negative = x < 0
        if is_negative:
            x *= -1
        result = 0
        while x > 0:
            result *= 10
            result += x % 10
            x = x // 10
        if is_negative:
            result *= -1
        return result


def tester():
    print("7 start")
    examples = [
        (123, 321),
        (-123, -321),
        (120, 21),
    ]
    solution = Solution()
    for example in examples:
        input_, exp = example
        output_ = solution.reverse(input_)
        assert exp == output_, f'\ninp: {input_}\nexp: {exp}\nout: {output_}'
    print("7 complete")
