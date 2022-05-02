from typing import List

from python3._2_AddTwoNumbers import ListNode, Solution


def convertToListNodes(l_in: List[int]) -> ListNode:
    head = ListNode(l_in[0])
    last = head
    for i in range(1, len(l_in)):
        next_ = ListNode(l_in[i])
        last.next = next_
        last = next_
    return head


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
