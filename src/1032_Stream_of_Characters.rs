// 1032. Stream of Characters
// https://leetcode.cn/problems/stream-of-characters/


struct StreamChecker {
    buffer: String,
    words: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {

    fn new(words: Vec<String>) -> Self {
        StreamChecker {
            words: words,
            buffer: "".to_string(),
        }
    }
    
    fn query(& mut self, letter: char) -> bool {
        self.buffer.push(letter);
        for str in self.words.iter() {
            if self.buffer.ends_with(str) {
                return true;
            }
        }

        false
    }
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */


/// 评论区佬
/// 字典树（Trie树）是一种树形结构，是一种哈希树的变种。
/// 典型应用于统计和排序大量的字符串（但不仅限于字符串），所以经常被搜索引擎系统用于文本词频统计。
/// 字典树的优点是：利用字符串的公共前缀来减少查询时间，最坏的情况下查询时间复杂度为O(m)，其中m为待查字符串的长度。
/// 在本题中，我们使用字典树来实现一个流式查询器，可以在O(n)的时间复杂度内查询一个字符串是否在给定的字符串集合中出现过。
/// 
/// 在代码中，我们定义了一个TrieNode结构体，表示字典树的节点。
/// 每个节点包含一个长度为26的Option<Box<TrieNode>>数组，表示该节点的26个子节点。is_word表示该节点是否为某个单词的结尾。
struct Trie<const N:usize> (Box<([Option<Trie<N>>;N],bool)>);

impl<const N:usize> Trie<N> {
    fn new() -> Self {
        Self (
            /// 需要注意的是，在这里使用了神奇技巧：由于Rust语言不支持大于32长度大小固定数组的初始化，
            /// 因此使用了`[();N]`来生成一个长度为N的空元组数组，
            /// 并通过map函数将其转换成了长度为N的None选项数组。
            Box::new(([();N].map(|_|None),false))
            // 这里Default::default()并不能初始化大于32的[None;N]
            // 而直接写[None;N]会因为None不是Copy而出错
            // 可以用泛型定义一个None常量，然后用[常量;N]初始化
            // 也可以像这里一样，使用一个神奇的技巧。
        )
    }

    /// 这段代码是一个 Trie 树的插入操作。Trie 树是一种树形数据结构，用于快速检索字符串集合中是否存在某个字符串。
    /// 该函数接受一个字符串作为参数，并将其插入到 Trie 树中。在这个过程中，它从根节点开始遍历 Trie 树，并沿着单词的每个字符向下移动。
    /// 如果当前字符对应的子节点不存在，则创建一个新的子节点并继续向下移动。最后，在单词的最后一个字符处，将末尾标记设置为 true。
    fn insert(&mut self, word: String) {
        let mut ptr = self;
        for c in word.bytes().rev() {
            if ptr.0.0[(c - b'a') as usize].is_none() {
                ptr.0.0[(c - b'a') as usize] = Some(Trie::new());
            }
            ptr = ptr.0.0[(c - b'a') as usize].as_mut().unwrap();
        }
        ptr.0.1 = true;
    }

    // 判断给定字节数组是否在Tire树中存在。
    fn search(&self, stream: &[u8]) -> bool {
        let mut ptr = self;
        for c in stream.iter().rev() {
            if ptr.0.0[(c - b'a') as usize].is_some() {
                ptr = ptr.0.0[(c - b'a') as usize].as_ref().unwrap();
                if ptr.0.1 {
                    return true;
                }
            } else {
                return false
            }
        }
        false
    }
}

struct StreamChecker {
    trie: Trie<26>,
    stream: Vec<u8>,
}

/**
 * `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl StreamChecker {

    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for s in words {
            trie.insert(s);
        }
        Self {
            trie: trie,
            stream: vec![],
        }
    }
    
    fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter as u8);
        self.trie.search(&self.stream)
    }
}

/**
 * Your StreamChecker object will be instantiated and called as such:
* let obj = StreamChecker::new(words);
* let ret_1: bool = obj.query(letter);
*/

