from collections import defaultdict
from typing import List

from python3.helper import Eg, tester_helper


class Node:
    def __init__(self):
        # root is one of:
        #   None: not yet visited
        #   -1: already processed
        #   non-negative integer: what Wikipedia pseudo code calls 'lowlink'
        self.root = None
        self.succ = []


class Solution:
    # Src: https://rosettacode.org/wiki/Tarjan#Python
    @staticmethod
    def from_edges(edges):
        """translate list of edges to list of nodes"""
        nodes = defaultdict(Node)
        for v, w in edges:
            nodes[v].succ.append(nodes[w])

        for i, v in nodes.items():  # name the nodes for final output
            v.id = i

        return nodes.values()

    @staticmethod
    def trajan(V):
        def strongconnect(v, S):
            v.root = pos = len(S)
            S.append(v)

            for w in v.succ:
                if w.root is None:  # not yet visited
                    yield from strongconnect(w, S)

                if w.root >= 0:  # still on stack
                    v.root = min(v.root, w.root)

            if v.root == pos:  # v is the root, return everything above
                res, S[pos:] = S[pos:], []
                for w in res:
                    w.root = -1
                yield [r.id for r in res]

        for v in V:
            if v.root is None:
                yield from strongconnect(v, [])

    def criticalConnections(self, n: int, connections: List[List[int]]) \
            -> List[List[int]]:
        for g in self.trajan(self.from_edges(connections)):
            print(g)
        print()


def tester():
    examples = [
        Eg((4, [[0, 1], [1, 2], [2, 0], [1, 3]]), [[1, 3]]),
        Eg((2, [[0, 1]]), [[0, 1]])
    ]
    tester_helper(1192, examples, Solution().criticalConnections)
