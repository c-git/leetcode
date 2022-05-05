class MyStack:

    def __init__(self):
        self.queue1 = MyQueue()
        self.queue2 = MyQueue()

    def push(self, x: int) -> None:
        self.queue1.push(x)

    def _get_top(self, keep: bool) -> int:
        """
        Gets the top value on the stack. If keep is true it is kept at the top
        :param keep: if true the value returned does not get removed from the
        stack
        :return: the top value on the stack
        """
        last = self.queue1.pop()
        while not self.queue1.is_empty():
            self.queue2.push(last)
            last = self.queue1.pop()
        if keep:
            self.queue2.push(last)
        self.queue1, self.queue2 = self.queue2, self.queue1
        return last

    def pop(self) -> int:
        return self._get_top(False)

    def top(self) -> int:
        return self._get_top(True)

    def empty(self) -> bool:
        return self.queue1.is_empty()


# Your MyStack object will be instantiated and called as such:
# obj = MyStack()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.top()
# param_4 = obj.empty()

class MyQueue:
    def __init__(self):
        self.data = []

    def push(self, value: int):
        self.data.insert(0, value)

    def peek(self) -> int:
        return self.data[-1]

    def pop(self) -> int:
        return self.data.pop()

    def size(self) -> int:
        return len(self.data)

    def is_empty(self) -> bool:
        return self.size() == 0


def tester():
    print("225 running")
    stack = MyStack()
    stack.push(1)
    stack.push(2)
    assert stack.top() == 2
    assert stack.pop() == 2
    assert not stack.empty()
    assert stack.pop() == 1
    assert stack.empty()
    print("225 complete")
