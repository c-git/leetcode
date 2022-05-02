from python3._2164_SortEvenandOddIndicesIndependently import Solution


def main():
    examples = [
        ([4, 1, 2, 3], [2, 3, 4, 1]),
        ([2, 1], [2, 1]),
    ]
    solution = Solution()
    for example in examples:
        input_, exp = example
        output_ = solution.sortEvenOdd(input_)
        assert exp == output_, \
            f'\n' \
            f'inp: {input_}\n' \
            f'exp: {exp}\n' \
            f'out: {output_}'


if __name__ == '__main__':
    main()
