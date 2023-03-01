// 205. Isomorphic Strings
// https://leetcode.cn/problems/isomorphic-strings/

use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map1: HashMap<char, char> = HashMap::new();
        let mut map2: HashMap<char, char> = HashMap::new();
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        if s_chars.len() != t_chars.len() {
            return false;
        }

        for i in 0..s_chars.len() {
            map1.insert(s_chars[i], t_chars[i]);
            map2.insert(t_chars[i], s_chars[i]);
        }

        let (mut s1, mut s2) = ("".to_string(), "".to_string());
        for i in 0..s.len() {
            if let Some(ch) = map1.get(&s_chars[i]) {
                s1.push(*ch);
            }
        }
        for i in 0..t.len() {
            if let Some(ch) = map2.get(&t_chars[i]) {
                s2.push(*ch);
            }
        }

        s1 == t && s2 == s
    }
}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut left_hash = vec![s.len(); 256];
        let mut right_hash = vec![t.len(); 256];

        for (i, (left, right)) in s.bytes().zip(t.bytes()).enumerate() {
            if left_hash[left as usize] != right_hash[right as usize] {
                return false;
            }
            left_hash[left as usize] = i;
            right_hash[right as usize] = i;
       }

        true
    }
}

// use std::collections::HashMap;
// let mut map = HashMap::new();
// assert_eq!(map.insert(37, "a"), None);
// assert_eq!(map.is_empty(), false);
// map.insert(37, "b");
// assert_eq!(map.insert(37, "c"), Some("b"));
// assert_eq!(map[&37], "c");
// If the map did not have this key present, None is returned.
// If the map did have this key present, the value is updated, and the old value is returned. 
// The key is not updated, though; this matters for types that can be == without being identical. 
// See the module-level documentation for more.
use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut ab = HashMap::new();
        let mut ba = HashMap::new();
        for (a, b) in s.chars().zip(t.chars()) {
            if let Some(c) = ab.insert(a, b) {
                if b != c {
                    return false;
                }
            }
            if let Some(c) = ba.insert(b, a) {
                if a != c {
                    return false;
                }
            }
        }
        true
    }
}

// 作者：tab-liu
// 链接：https://leetcode.cn/problems/isomorphic-strings/solution/205-tong-gou-zi-fu-chuan-by-tab-liu-qrd7/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

// class Solution {
//     public:
//         bool isIsomorphic(string s, string t) {
//             unordered_map<char, char> s2t;
//             unordered_map<char, char> t2s;
//             int len = s.length();
//             for (int i = 0; i < len; ++i) {
//                 char x = s[i], y = t[i];
//                 if ((s2t.count(x) && s2t[x] != y) || (t2s.count(y) && t2s[y] != x)) {
//                     return false;
//                 }
//                 s2t[x] = y;
//                 t2s[y] = x;
//             }
//             return true;
//         }
//     };
    
//     作者：LeetCode-Solution
//     链接：https://leetcode.cn/problems/isomorphic-strings/solution/tong-gou-zi-fu-chuan-by-leetcode-solutio-s6fd/
//     来源：力扣（LeetCode）
//     著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。