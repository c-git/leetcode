from python3._905_SortArrayByParity import Solution


def main():
    examples = [
        ([3, 1, 2, 4], [2, 4, 3, 1]),
        ([0], [0]),
    ]
    solution = Solution()
    for example in examples:
        input_, output_ = example
        assert solution.sortArrayByParity(input_) == output_, \
            f'in: {input_}\nout:{output_}'


if __name__ == '__main__':
    main()
