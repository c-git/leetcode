from typing import List, Optional, Union

"""
This is the interface that allows for creating nested lists.
You should not implement it, or speculate about its implementation
"""


class NestedInteger:
    """
    Testing implementation
    """

    def __init__(self, value: Union[int, List]):
        self.value: Union[int, List, None] = None
        if isinstance(value, int):
            self.value = value
        else:
            self.value = [NestedInteger(val) for val in value]

    def isInteger(self) -> bool:
        """
        @return True if this NestedInteger holds a single integer, rather
        than a nested list.
        """
        return isinstance(self.value, int)

    def getInteger(self) -> Optional[int]:
        """
        @return the single integer that this NestedInteger holds,
        if it holds a single integer
        Return None if this NestedInteger holds a nested list
        """
        if self.isInteger():
            return self.value
        else:
            return None

    def __str__(self):
        if self.isInteger():
            return str(self.getInteger())
        else:
            return str(self.getList())

    def __repr__(self):
        return str(self)

    def getList(self) -> Optional[List]:
        """
        @return the nested list that this NestedInteger holds, if it holds a
        nested list
        Return None if this NestedInteger holds a single integer
        """
        if self.isInteger():
            return None
        else:
            return self.value


class NestedIterator:
    def __init__(self, nestedList: List[NestedInteger]):
        self.next_iter = self.next_gen(nestedList)
        self.next_val: Optional[int] = next(self.next_iter)

    def next_gen(self, nestedList: List[NestedInteger]):
        if isinstance(nestedList, NestedInteger):
            if nestedList.isInteger():
                yield nestedList.getInteger()
                return
            else:
                nestedList = nestedList.getList()
        for nInt in nestedList:
            if nInt.isInteger():
                yield nInt.getInteger()
            else:
                for val in self.next_gen(nInt.getList()):
                    yield val

    def next(self) -> int:
        assert self.next_val is not None
        result = self.next_val
        try:
            self.next_val = next(self.next_iter)
        except StopIteration:
            self.next_val = None
        return result

    def hasNext(self) -> bool:
        return self.next_val is not None


# Your NestedIterator object will be instantiated and called as such:
# i, v = NestedIterator(nestedList), []
# while i.hasNext(): v.append(i.next())

def flatten(nestedList):
    nestedList = [NestedInteger(x) for x in nestedList]
    i, v = NestedIterator(nestedList), []
    while i.hasNext():
        v.append(i.next())
    return v


def tester():
    print("341 start")
    examples = [
        ([[1, 1], 2, [1, 1]], [1, 1, 2, 1, 1]),
        ([1, [4, [6]]], [1, 4, 6]),
    ]
    for example in examples:
        input_, exp = example
        output_ = flatten(input_)
        assert exp == output_, f'\ninp: {input_}\nexp: {exp}\nout: {output_}'
    print("341 complete")
