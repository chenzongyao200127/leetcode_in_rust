// 2296_设计一个文本编辑器
// https://leetcode.cn/problems/design-a-text-editor/description/

struct TextEditor {
    text: String,
    cursor: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TextEditor {
    fn new() -> Self {
        TextEditor {
            text: String::new(),
            cursor: 0,
        }
    }

    fn add_text(&mut self, text: String) {
        self.text.insert_str(self.cursor, &text);
        self.cursor += text.len();
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let k = k as usize;
        let n = self.text.len();
        let k = if k > n { n } else { k };
        let k = if k > self.cursor { self.cursor } else { k };
        self.text.drain(self.cursor - k..self.cursor);
        self.cursor -= k;
        k as i32
    }

    fn cursor_left(&mut self, k: i32) -> String {
        let k = k as usize;
        let k = if k > self.cursor { self.cursor } else { k };
        self.cursor -= k;
        let len = self.cursor.min(10);
        self.text[self.cursor - len..self.cursor].to_string()
    }

    fn cursor_right(&mut self, k: i32) -> String {
        let mut k = k as usize;
        let n = self.text.len();
        if k > n - self.cursor {
            k = n - self.cursor;
        }
        self.cursor += k;
        let len = (self.cursor).min(10);
        self.text[self.cursor - len..self.cursor].to_string()
    }
}

// 如果 k 很大，要怎么做？有没有复杂度和 k 无关的算法？
// 可以用 Splay 模拟文本的添加和删除。感兴趣的同学可以查阅相关资料。
// Splay 树
//
