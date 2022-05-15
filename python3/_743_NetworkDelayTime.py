from typing import Dict, List, Tuple

from python3.helper import Eg, tester_helper


class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        targets = set(range(1, n + 1))

        # src -> (time, dst)
        links: Dict[int, List[Tuple[int, int]]] = {}
        for target in targets:
            links[target] = []
        for src, dst, time in times:
            links[src].append((time, dst))

        reachable = [(0, k)]  # Reachable nodes sorted by travel time
        max_travel = 0
        while len(reachable) > 0 and len(targets) > 0:
            curr_dist, node = reachable.pop(0)
            max_travel = max(curr_dist, max_travel)
            if node in targets:
                targets.remove(node)  # Remove from targets it has been reached
                # New node add edges
                for edge_time, dst in links[node]:
                    reachable.append((edge_time + curr_dist, dst))
                reachable.sort()

        if len(targets) > 0:
            return -1
        else:
            return max_travel


def tester():
    examples = [
        Eg(([[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2), 2),
        Eg(([[2, 1, 1], [2, 4, 5], [2, 3, 1], [3, 4, 1]], 4, 2), 2),
        Eg(([[1, 2, 1]], 2, 1), 1),
        Eg(([[1, 2, 1]], 2, 2), -1),
    ]
    tester_helper(1641, examples, Solution().networkDelayTime)
