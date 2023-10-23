// 232_Implement_Queue_using_Stacks
// https://leetcode.cn/problems/implement-queue-using-stacks/description/

// 请你仅使用两个栈实现先入先出队列。队列应当支持一般队列支持的所有操作（push、pop、peek、empty）：

// 实现 MyQueue 类：

// void push(int x) 将元素 x 推到队列的末尾
// int pop() 从队列的开头移除并返回元素
// int peek() 返回队列开头的元素
// boolean empty() 如果队列为空，返回 true ；否则，返回 false

// 说明：
// 你只能使用标准的栈操作 —— 也就是只有 push to top, peek/pop from top, size, 和 is empty 操作是合法的。
// 你所使用的语言也许不支持栈。你可以使用 list 或者 deque（双端队列）来模拟一个栈，只要是标准的栈操作即可。

// 示例 1：

// 输入：
// ["MyQueue", "push", "push", "peek", "pop", "empty"]
// [[], [1], [2], [], [], []]
// 输出：
// [null, null, null, 1, 1, false]

// 解释：
// MyQueue myQueue = new MyQueue();
// myQueue.push(1); // queue is: [1]
// myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
// myQueue.peek(); // return 1
// myQueue.pop(); // return 1, queue is [2]
// myQueue.empty(); // return false
 

// 提示：

// 1 <= x <= 9
// 最多调用 100 次 push、pop、peek 和 empty
// 假设所有操作都是有效的 （例如，一个空的队列不会调用 pop 或者 peek 操作）
 

// 进阶：

// 你能否实现每个操作均摊时间复杂度为 O(1) 的队列？换句话说，执行 n 个操作的总时间复杂度为 O(n) ，即使其中一个操作可能花费较长时间。

struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        MyQueue {
            s1: Vec::new(),
            s2: Vec::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        while !self.s1.is_empty() {
            self.s2.push(self.s1.pop().unwrap());
        }
        self.s1.push(x);
        while !self.s2.is_empty() {
            self.s1.push(self.s2.pop().unwrap());
        }
    }
    
    fn pop(&mut self) -> i32 {
        self.s1.pop().unwrap()
    }
    
    fn peek(&self) -> i32 {
        self.s1.last().copied().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.s1.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>
}

// Your implementation of a queue using two stacks seems to follow the "costly push" approach where each push operation ensures that s1 always maintains the front of the queue at its top. However, this method can be optimized for better average performance.

// Here's a "costly pop" approach:

// Pushes go to s1 directly.
// Pops and peeks are performed on s2. If s2 is empty, then we pour everything from s1 into s2.

// This optimization takes advantage of the fact that moving all elements from s1 to s2 is only needed when s2 is empty. This can reduce the average number of operations per pop or peek.

// Additionally, for safety, it's better to use Option<i32> for methods like pop and peek to handle the case when the queue is empty.

// Here's the optimized code:

impl MyQueue {

    fn new() -> Self {
        MyQueue {
            s1: Vec::new(),
            s2: Vec::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        self.s1.push(x);
    }
    
    fn pop(&mut self) -> Option<i32> {
        if self.s2.is_empty() {
            while let Some(val) = self.s1.pop() {
                self.s2.push(val);
            }
        }
        self.s2.pop()
    }
    
    fn peek(&self) -> Option<i32> {
        if !self.s2.is_empty() {
            self.s2.last().copied()
        } else {
            self.s1.first().copied()
        }
    }
    
    fn empty(&self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }
}
