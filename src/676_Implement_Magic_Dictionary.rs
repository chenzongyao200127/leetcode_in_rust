// 676_Implement_Magic_Dictionary
// https://leetcode.cn/problems/implement-magic-dictionary/description/

use std::collections::HashSet;

struct MagicDictionary {
    set: HashSet<String>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    fn new() -> Self {
        MagicDictionary {
            set: HashSet::new()
        }
    }
    
    fn build_dict(&mut self, dictionary: Vec<String>) {
        self.set = dictionary.into_iter().collect();
    }
    
    fn search(&self, search_word: String) -> bool {
        self.set.iter().any(|word| {
            if word.len() == search_word.len() {
                word.chars().zip(search_word.chars()).filter(|(a, b)| a != b).count() == 1
            } else {
                false
            }
        })
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */