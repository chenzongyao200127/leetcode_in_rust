// 622_Design_Circular_Queue
// https://leetcode.cn/problems/design-circular-queue/

// 使用数组模拟循环队列

struct MyCircularQueue {
    queue: Vec<i32>,
    front: usize,
    rear: usize,
    count: usize,
    capacity: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            queue: vec![0; k as usize],
            front: 0,
            rear: 0,
            count: 0,
            capacity: k as usize,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.queue[self.rear] = value;
        self.rear = (self.rear + 1) % self.capacity;
        self.count += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.capacity;
        self.count -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.queue[self.front]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.queue[(self.rear + self.capacity - 1) % self.capacity]
        }
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn is_full(&self) -> bool {
        self.count == self.capacity
    }
}