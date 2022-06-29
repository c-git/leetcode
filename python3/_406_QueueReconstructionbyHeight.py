from typing import List

from python3.helper import Eg, tester_helper


class Solution:
    def reconstructQueue(self, people: List[List[int]]) -> List[List[int]]:
        people.sort()
        # ASSUME Last person is already in the correct position after sorting
        # as we can move them no further back
        for i in range(len(people) - 2, -1, -1):
            # LI
            # - All the people greater than i are in their optimal position
            #   given the information available.
            person = people[i]  # Person to be put in correct position
            if person[1] > 0:
                # Requires people the same height or taller in front
                # therefore they may need to move back unless there are
                # enough people the same height ahead of them

                # Count number of people the same height ahead of the person
                ahead = self.same_height_ahead(people, i)

                # move this person backward
                moves_needed = person[1] - ahead
                assert moves_needed >= 0  # Assumed based on guarantee in given
                pos = i + 1
                while moves_needed > 0:
                    assert pos < len(people)
                    people[pos - 1] = people[pos]
                    moves_needed -= 1
                    pos += 1
                people[pos - 1] = person
        return people

    def same_height_ahead(self, people: List[List[int]], pos: int) -> int:
        height = people[pos][0]
        i = pos - 1
        result = 0
        while i >= 0 and people[i][0] == height:
            result += 1
            i -= 1
        return result


def tester():
    examples = [
        Eg([[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]],
           [[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]]),
        Eg([[6, 0], [5, 0], [4, 0], [3, 2], [2, 2], [1, 4]],
           [[4, 0], [5, 0], [2, 2], [3, 2], [1, 4], [6, 0]])
    ]
    tester_helper(406, examples, Solution().reconstructQueue)
