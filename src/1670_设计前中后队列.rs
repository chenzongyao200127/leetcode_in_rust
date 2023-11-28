struct FrontMiddleBackQueue {
    queue: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {

    fn new() -> Self {
        FrontMiddleBackQueue { queue: vec![] }
    }
    
    fn push_front(&mut self, val: i32) {
        self.queue.insert(0, val);
    }
    
    fn push_middle(&mut self, val: i32) {
        let idx = self.queue.len() / 2;
        self.queue.insert(idx, val);
    }
    
    fn push_back(&mut self, val: i32) {
        self.queue.push(val);
    }
    
    fn pop_front(&mut self) -> i32 {
        if self.queue.is_empty() {
            -1
        } else {
            Some(self.queue.remove(0)).unwrap()
        }
    }
    
    fn pop_middle(&mut self) -> i32 {
        if self.queue.is_empty() {
            -1
        } else {
            let len = self.queue.len();
            let idx = (len - 1) / 2;
            Some(self.queue.remove(idx)).unwrap()
        }
    }
    
    fn pop_back(&mut self) -> i32 {
        self.queue.pop().unwrap()
    }
}

/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */



use std::collections::VecDeque;

struct FrontMiddleBackQueue {
    left: VecDeque<i32>,
    right: VecDeque<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self {
        FrontMiddleBackQueue {
            left: VecDeque::new(),
            right: VecDeque::new(),
        }
    }

    // 两个双端队列 + Balance 方法
    // 调整长度，保证 0 <= right.len() - left.len() <= 1
    // 从而保证可以在正中间插入删除元素
    fn balance(&mut self) {
        if self.left.len() > self.right.len() {
            self.right.push_front(self.left.pop_back().unwrap());
        } else if self.right.len() > self.left.len() + 1 {
            self.left.push_back(self.right.pop_front().unwrap());
        }
    }

    fn push_front(&mut self, val: i32) {
        self.left.push_front(val);
        self.balance();
    }

    fn push_middle(&mut self, val: i32) {
        if self.left.len() < self.right.len() {
            self.left.push_back(val);
        } else {
            self.right.push_front(val);
        }
    }

    fn push_back(&mut self, val: i32) {
        self.right.push_back(val);
        self.balance();
    }

    fn pop_front(&mut self) -> i32 {
        if self.right.is_empty() { // 整个队列为空
            return -1;
        }
        let val = if self.left.is_empty() {
            self.right.pop_front().unwrap()
        } else {
            self.left.pop_front().unwrap()
        };
        self.balance();
        val
    }

    fn pop_middle(&mut self) -> i32 {
        if self.right.is_empty() { // 整个队列为空
            return -1;
        }
        if self.left.len() == self.right.len() {
            return self.left.pop_back().unwrap();
        }
        self.right.pop_front().unwrap()
    }

    fn pop_back(&mut self) -> i32 {
        if self.right.is_empty() { // 整个队列为空
            return -1;
        }
        let val = self.right.pop_back().unwrap();
        self.balance();
        val
    }
}
