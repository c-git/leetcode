# Src: https://rosettacode.org/wiki/Tarjan#Python

# Src: https://leetcode.com/problems/critical-connections-in-a-network
# /discuss/2050090/Python3-oror-Trajan-algorithm-%2B-Vidoe-Description
from collections import defaultdict
from typing import Dict, List, Optional

from python3.helper import Eg, tester_helper


class Node:
    def __init__(self):
        self.rank = None
        self.succ = []
        self.id = None  # to uniquely identify a node (from input values)

    def __repr__(self):
        return f'{self.id} - (R: {self.rank}, S:{[x.id for x in self.succ]})'


class Solution:
    @staticmethod
    def from_edges(edges: List[List[int]]):
        """translate list of edges to list of nodes"""
        result = defaultdict(Node)
        for v, w in edges:
            result[v].succ.append(result[w])
            result[w].succ.append(result[v])

        for i, v in result.items():
            v.id = i

        return result

    def dfs(self, node: Node, rank: int, parent: Optional[Node] = None):
        if node.rank is not None:
            return  # Exit without yielding any values

        node.rank = rank
        for adj in node.succ:
            if adj == parent:
                continue  # Skip this node, this is where we just came from

            if adj.rank is None:
                # Use deque to consume iterator
                yield from self.dfs(adj, rank + 1, node)

            node.rank = min(node.rank, adj.rank)

            if adj.rank > rank:
                # We found a critical edge
                yield [node.id, adj.id]

    def criticalConnections(self, n: int, connections: List[List[int]]) \
            -> List[List[int]]:
        graph: Dict[int, Node] = self.from_edges(connections)
        return list(self.dfs(graph[0], 0))


def tester():
    examples = [
        Eg((2, [[0, 1], [1, 2], [2, 0]]), []),
        Eg((4, [[4, 1], [0, 1], [1, 2], [2, 0], [1, 3]]), [[1, 4], [1, 3]]),
        Eg((4, [[0, 1], [1, 2], [2, 0], [1, 3]]), [[1, 3]]),
        Eg((2, [[0, 1]]), [[0, 1]]),
    ]
    tester_helper(1192, examples, Solution().criticalConnections)
