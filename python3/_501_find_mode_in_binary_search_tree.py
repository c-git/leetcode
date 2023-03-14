# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
from typing import Optional, List, Dict

from python3.helper import Eg, tester_helper, int_list_to_tree, evaluator_any_order_list
from python3.helper_classes import TreeNode


class Solution:
    def get_frequency_distribution(self, root: Optional[TreeNode]) -> Dict[int, int]:
        if root is None:
            return {}

        # Initialize result with values from left child
        result = self.get_frequency_distribution(root.left)

        # Get values from right child
        right_freq = self.get_frequency_distribution(root.right)

        # Join right child to result
        for key in right_freq:
            if result.get(key) is None:
                result[key] = 0
            result[key] += right_freq[key]

        # Include current node
        if result.get(root.val) is None:
            result[root.val] = 0
        result[root.val] += 1

        return result

    def findMode(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []

        frequency_distribution = self.get_frequency_distribution(root)

        assert len(frequency_distribution) > 0, "Should have had at least 1 value because root was not None"

        # Get the highest frequency
        highest_freq = 0
        for key in frequency_distribution:
            if highest_freq < frequency_distribution[key]:
                highest_freq = frequency_distribution[key]

        result = []
        for key in frequency_distribution:
            if frequency_distribution[key] == highest_freq:
                result.append(key)

        return result


def tester():
    examples = [
        Eg(int_list_to_tree([1, None, 2, 2, None]), [2], evaluator_any_order_list),
        Eg(int_list_to_tree([0]), [0], evaluator_any_order_list),
    ]
    tester_helper(501, examples, Solution().findMode)
