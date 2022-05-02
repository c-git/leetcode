from python3._905_SortArrayByParity import Solution


def main():
    examples = [
        ([3, 1, 2, 4], [2, 4, 3, 1]),
        ([0], [0]),
    ]
    solution = Solution()
    for example in examples:
        input_, exp = example
        output_ = solution.sortArrayByParity(input_)
        assert exp == output_, \
            f'\n' \
            f'inp: {input_}\n' \
            f'exp: {exp}\n' \
            f'out: {output_}'


if __name__ == '__main__':
    main()
