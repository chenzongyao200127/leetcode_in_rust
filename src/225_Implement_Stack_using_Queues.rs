// 225_Implement_Stack_using_Queues
// https://leetcode.cn/problems/implement-stack-using-queues/description/

// 请你仅使用两个队列实现一个后入先出（LIFO）的栈，并支持普通栈的全部四种操作（push、top、pop 和 empty）。

// 实现 MyStack 类：

//  void push(int x) 将元素 x 压入栈顶。
//  int pop() 移除并返回栈顶元素。
//  int top() 返回栈顶元素。
//  boolean empty() 如果栈是空的，返回 true ；否则，返回 false 。
 

// 注意：
// 你只能使用队列的基本操作 —— 也就是 push to back、peek/pop from front、size 和 is empty 这些操作。
// 你所使用的语言也许不支持队列。 你可以使用 list （列表）或者 deque（双端队列）来模拟一个队列 , 只要是标准的队列操作即可。


// 示例：

// 输入：
// ["MyStack", "push", "push", "top", "pop", "empty"]
// [[], [1], [2], [], [], []]
// 输出：
// [null, null, null, 2, 2, false]

// 解释：
// MyStack myStack = new MyStack();
// myStack.push(1);
// myStack.push(2);
// myStack.top(); // 返回 2
// myStack.pop(); // 返回 2
// myStack.empty(); // 返回 False
 

// 提示：

// 1 <= x <= 9
// 最多调用100 次 push、pop、top 和 empty
// 每次调用 pop 和 top 都保证栈不为空

// 进阶：你能否仅用一个队列来实现栈。
struct MyStack {
    stk: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    fn new() -> Self {
        MyStack {
            stk: Vec::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        self.stk.push(x)
    }
    
    fn pop(&mut self) -> i32 {
        let top = self.stk[self.stk.len()-1];
        self.stk.pop();
        return top
    }
    
    fn top(&self) -> i32 {
        self.stk[self.stk.len()-1]
    }
    
    fn empty(&self) -> bool {
        self.stk.is_empty()
    }
}


struct MyStack {
    stk: Vec<i32>
}

impl MyStack {

    fn new() -> Self {
        MyStack {
            stk: Vec::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        self.stk.push(x)
    }
    
    fn pop(&mut self) -> i32 {
        self.stk.pop().unwrap()
    }
    
    fn top(&self) -> i32 {
        self.stk.last().copied().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.stk.is_empty()
    }
}


/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */