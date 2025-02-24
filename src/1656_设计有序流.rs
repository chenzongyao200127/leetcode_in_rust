// 1656. 设计有序流
// https://leetcode.cn/problems/design-an-ordered-stream/description/

// 有 n 个 (id, value) 对，其中 id 是 1 到 n 之间的一个整数，value 是一个字符串。不存在 id 相同的两个 (id, value) 对。

// 设计一个流，以 任意 顺序获取 n 个 (id, value) 对，并在多次调用时 按 id 递增的顺序 返回一些值。

// 实现 OrderedStream 类：

// OrderedStream(int n) 构造一个能接收 n 个值的流，并将当前指针 ptr 设为 1 。
// String[] insert(int id, String value) 向流中存储新的 (id, value) 对。存储后：
// 如果流存储有 id = ptr 的 (id, value) 对，则找出从 id = ptr 开始的 最长 id 连续递增序列 ，并 按顺序 返回与这些 id 关联的值的列表。然后，将 ptr 更新为最后那个  id + 1 。
// 否则，返回一个空列表。

use std::collections::HashMap;

struct OrderedStream {
    ptr: i32,
    map: HashMap<i32, String>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {

    fn new(n: i32) -> Self {
        OrderedStream {
            ptr: 1,
            map: HashMap::new(),
        }
    }
    
    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        // first store the new pair
        self.map.insert(id_key, value);

        // check if (id, value) pair is in the map
        if let Some(_) = self.map.get(&self.ptr) {
            let mut res = vec![];
            while let Some(v) = self.map.get(&self.ptr) {
                res.push(v.clone());
                self.ptr += 1;
            }
            return res;
        } else {
            return vec![];
        }

        // update ptr
        self.ptr += 1;
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */