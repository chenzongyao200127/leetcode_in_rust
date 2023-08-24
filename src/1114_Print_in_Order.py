# 1114. Print in Order
# https://leetcode.cn/problems/print-in-order/

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

from typing import Callable
from time import sleep
class Foo:
    def __init__(self):
        self.t = 0

    def first(self, printFirst: 'Callable[[], None]') -> None:
        printFirst()
        self.t = 1

    def second(self, printSecond: 'Callable[[], None]') -> None:
        while self.t != 1: 
            sleep(1e-3)
        printSecond()
        self.t = 2

    def third(self, printThird: 'Callable[[], None]') -> None:
        while self.t != 2: 
            sleep(1e-3)
        printThird()
        
        
# 9 种方法
# https://leetcode.cn/problems/print-in-order/solutions/30024/1114-an-xu-da-yin-python3de-5chong-jie-fa-by-tuotu/

# 第一种基础方法超时，中间五种方法在线上测试的时间上基本没有区别都可以做到最快72ms, 95-97%这样。

# 最后几种数据结构的方法没有多线程模块threading的阻塞，分别用的是多线程专用的阻塞队列数据结构queue的两种方法，
# 和本身线程安全的字典，队列最快可以达到68ms, 98%这样，字典最快可以达到64ms, 99%这样，
# 对于不懂多线程的同学来说也算是比较好理解的方法了。（11月更新一下，字典法现在被判不在规定的线程输出，已经不能用了）

# 另外sleep大法已经不能过了，包括threading模块里的Timer也一样，
# 主要是每个函数的执行时间不固定，休眠时间短未必能做到正解，休眠时间太久又会超时。



# 方法一，while循环法（超时）：
# 可能是不懂多线程的同学最能够接受的基础解法，可以大体理解多线程的阻塞是什么意思。
# 就相当于先用某些方法卡住执行顺序，然后不断监控目标，直到目标符合条件时才跳出当前断点继续执行后续语句。
# 输出是正确的，只是因为没法像threading模块那样很好的监控线程，所以大概率会超时，其他语言或许可以用这种方法AC，
# 但python相对较慢，大约只能过30/37的数据。
# 对于单次阻塞来说，运行时间大约是threading模块时间的10-14倍这样，整个程序平均时间差距就会在15-25倍这样。

class Foo:
    def __init__(self):
        self.t = 0

    def first(self, printFirst: 'Callable[[], None]') -> None:
        printFirst()
        self.t = 1

    def second(self, printSecond: 'Callable[[], None]') -> None:
        while self.t != 1: 
            pass
        printSecond()
        self.t = 2

    def third(self, printThird: 'Callable[[], None]') -> None:
        while self.t != 2: 
            pass
        printThird()

# 来自评论区，改进后可以通过。 while循环跑满CPU，会影响GIL线程上下文切换的判定，可能是导致超时的重要原因之一，
# time.sleep会把CPU交还给GIL，可以让GIL及时切换线程，即使sleep很短的时间（比如1e-9或更小），也可以通过线上测试。

from time import sleep
class Foo:
    def __init__(self):
        self.t = 0

    def first(self, printFirst: 'Callable[[], None]') -> None:
        printFirst()
        self.t = 1

    def second(self, printSecond: 'Callable[[], None]') -> None:
        while self.t != 1: 
            sleep(1e-3)
        printSecond()
        self.t = 2

    def third(self, printThird: 'Callable[[], None]') -> None:
        while self.t != 2: 
            sleep(1e-3)
        printThird()

# 方法二，Condition条件对象法：

# threading模块里的Condition方法，后面五种的方法也都是调用这个模块和使用不同的方法了，方法就是启动wait_for来阻塞每个函数，
# 直到指示self.t为目标值的时候才释放线程，with是配合Condition方法常用的语法糖，主要是替代try语句的。

# 使用Python中的`threading`库，我们可以使用条件变量来进行同步。条件变量提供了一种机制，使一个线程等待直到特定条件满足，然后线程可以被唤醒并继续执行。

import threading

class Foo:
    def __init__(self):
        self.t = 0
        self.lock = threading.Lock()
        self.condition = threading.Condition(self.lock)

    def first(self, printFirst: 'Callable[[], None]') -> None:
        with self.lock:
            printFirst()
            self.t = 1
            self.condition.notify_all()

    def second(self, printSecond: 'Callable[[], None]') -> None:
        with self.lock:
            while self.t != 1:
                self.condition.wait()
            printSecond()
            self.t = 2
            self.condition.notify_all()

    def third(self, printThird: 'Callable[[], None]') -> None:
        with self.lock:
            while self.t != 2:
                self.condition.wait()
            printThird()

# 这里的关键部分是：

# - `self.condition.wait()`: 使当前线程等待，
# 直到某个其他线程调用`self.condition.notify()`（或`notify_all()`）来唤醒它。
# - `self.condition.notify_all()`: 唤醒所有在该条件上等待的线程。
 
# 这种方式比之前的轮询+休眠方法更加高效，因为线程只在必要的时候被唤醒。
# 在上述代码中，取互斥锁和释放互斥锁的过程体现在`with self.lock:`语句块中。

# 具体来说：

# - 当你进入`with self.lock:`块时，锁`self.lock`被自动获取（或者说，该锁被“锁定”）。
# - 当你退出这个块时，锁`self.lock`被自动释放（或者说，该锁被“解锁”）。

# 这是Python的上下文管理器特性。当你使用一个对象（在本例中为锁对象）作为`with`语句的上下文管理器时，
# 该对象的`__enter__`方法会被调用（在本例中，该方法获取锁），
# 当退出`with`块时，`__exit__`方法会被调用（在本例中，该方法释放锁）。

# 所以这种代码结构：

# with self.lock:
#     # critical section code

# 确保了所谓的"critical section"代码块在任何时候只被一个线程执行，从而提供了互斥性。



# 方法三，两把Lock锁对象法：

# 在这题里面功能都是类似的，就是添加阻塞，然后释放线程，只是类初始化的时候不能包含有参数，
# 所以要写一句acquire进行阻塞，调用其他函数的时候按顺序release释放。

import threading

class Foo:
    def __init__(self):
        self.l1 = threading.Lock()
        self.l1.acquire()
        self.l2 = threading.Lock()
        self.l2.acquire()

    def first(self, printFirst: 'Callable[[], None]') -> None:
        printFirst()
        self.l1.release()

    def second(self, printSecond: 'Callable[[], None]') -> None:
        self.l1.acquire()
        printSecond()
        self.l2.release()

    def third(self, printThird: 'Callable[[], None]') -> None:
        self.l2.acquire()
        printThird()
        

# 方法四，Semaphore信号量对象法：
# 和方法三是类似的，不过在类赋值的时候可以带有参数自带阻塞。
import threading

class Foo:
    def __init__(self):
        self.s1 = threading.Semaphore(0)
        self.s2 = threading.Semaphore(0)

    def first(self, printFirst: 'Callable[[], None]') -> None:
        printFirst()
        self.s1.release() # V 操作

    def second(self, printSecond: 'Callable[[], None]') -> None:
        self.s1.acquire() # P 操作
        printSecond()
        self.s2.release() # V 操作

    def third(self, printThird: 'Callable[[], None]') -> None:
        self.s2.acquire()
        printThird()
