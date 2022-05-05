from python3._2_AddTwoNumbers import Solution, convertToListNodes


def main():
    examples = [
        (([2, 4, 3], [5, 6, 4]), [7, 0, 8]),
        (([0], [0]), [0]),
        (([9, 9, 9, 9, 9, 9, 9], [9, 9, 9, 9]), [8, 9, 9, 9, 0, 0, 0, 1]),
    ]
    solution = Solution()
    for example in examples:
        input_, exp = example
        l1, l2 = input_
        l1 = convertToListNodes(l1)
        l2 = convertToListNodes(l2)
        exp = convertToListNodes(exp)
        output_ = solution.addTwoNumbers(l1, l2)
        assert exp == output_, \
            f'\n' \
            f'inp: {input_}\n' \
            f'exp: {exp}\n' \
            f'out: {output_}'


if __name__ == '__main__':
    main()
