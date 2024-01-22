# 1114_按序打印
# https://leetcode.cn/problems/print-in-order/description/

# 给你一个类：

# public class Foo {
#   public void first() { print("first"); }
#   public void second() { print("second"); }
#   public void third() { print("third"); }
# }
# 三个不同的线程 A、B、C 将会共用一个 Foo 实例。

# 线程 A 将会调用 first() 方法
# 线程 B 将会调用 second() 方法
# 线程 C 将会调用 third() 方法
# 请设计修改程序，以确保 second() 方法在 first() 方法之后被执行，third() 方法在 second() 方法之后被执行。

# 提示：

# 尽管输入中的数字似乎暗示了顺序，但是我们并不保证线程在操作系统中的调度顺序。
# 你看到的输入格式主要是为了确保测试的全面性。

from threading import Event


class Foo:
    s = 3

    def __init__(self):
        self.s = 3

    def first(self, printFirst: 'Callable[[], None]') -> None:
        while self.s != 3:
            continue
        # printFirst() outputs "first". Do not change or remove this line.
        printFirst()
        self.s -= 1

    def second(self, printSecond: 'Callable[[], None]') -> None:
        while self.s != 2:
            continue
        # printSecond() outputs "second". Do not change or remove this line.
        printSecond()
        self.s -= 1

    def third(self, printThird: 'Callable[[], None]') -> None:
        while self.s != 1:
            continue
        # printThird() outputs "third". Do not change or remove this line.
        printThird()


class Foo:
    def __init__(self):
        self.first_done = Event()
        self.second_done = Event()

    def first(self, printFirst: 'Callable[[], None]') -> None:
        # printFirst() outputs "first". Do not change or remove this line.
        printFirst()
        self.first_done.set()

    def second(self, printSecond: 'Callable[[], None]') -> None:
        self.first_done.wait()  # Wait for first() to finish
        # printSecond() outputs "second". Do not change or remove this line.
        printSecond()
        self.second_done.set()

    def third(self, printThird: 'Callable[[], None]') -> None:
        self.second_done.wait()  # Wait for second() to finish
        # printThird() outputs "third". Do not change or remove this line.
        printThird()
