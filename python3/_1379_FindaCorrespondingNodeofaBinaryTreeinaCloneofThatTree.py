from python3.helper import Eg, int_list_to_tree, tester_helper


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return self.val == other.val
        else:
            return False


class Solution:
    def getTargetCopy(self, original: TreeNode, cloned: TreeNode,
                      target: TreeNode) -> TreeNode:
        # ASSUMPTION original and cloned are value equal
        if target == original:
            return cloned
        if original is None:
            return None

        left = self.getTargetCopy(original.left, cloned.left, target)
        if left is not None:
            return left

        right = self.getTargetCopy(original.right, cloned.right, target)
        return right  # If None still what I would have had to return


def tester():
    examples = [
        Eg((
            int_list_to_tree([7, 4, 3, None, None, 6, 19], TreeNode),
            int_list_to_tree([7, 4, 3, None, None, 6, 19], TreeNode),
            int_list_to_tree([3], TreeNode)),
            int_list_to_tree([3], TreeNode)),
        Eg((
            int_list_to_tree([7], TreeNode),
            int_list_to_tree([7], TreeNode),
            int_list_to_tree([7], TreeNode)),
            int_list_to_tree([7], TreeNode)),
        Eg((
            int_list_to_tree(
                [8, None, 6, None, 5, None, 4, None, 3, None, 2, None, 1],
                TreeNode),
            int_list_to_tree(
                [8, None, 6, None, 5, None, 4, None, 3, None, 2, None, 1],
                TreeNode),
            int_list_to_tree([4], TreeNode)),
            int_list_to_tree([4], TreeNode)),
    ]
    tester_helper(1379, examples, Solution().getTargetCopy)
