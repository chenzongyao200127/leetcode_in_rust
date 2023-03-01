// 591. Tag Validator
// https://leetcode.cn/problems/tag-validator/


use std::collections::VecDeque;

// 详细思路见题解
impl Solution {
    pub fn is_valid(code: String) -> bool {
        let (mut i, n) = (0, code.len());
        let mut queue = VecDeque::new();

        while i < n {
            if i > 0 && queue.is_empty() {
                return false;
            }
            if is_starts_with(&code, "<![CDATA[", i) {
                let j = index_of(&code, "]]>", i+9);
                if j == -1 {
                    return false;
                }
                i = j as usize + 3;
            } else if is_starts_with(&code, "</", i) {
                let j = index_of(&code, ">", i+2);
                if j == -1 || i + 2 == j as usize || j as usize - i - 2 > 9 {
                    return false;
                }
                if queue.is_empty() || &code[i+2..j as usize] != *queue.back().unwrap() {
                    return false;
                }
                queue.pop_back();
                i = j as usize + 1;
            } else if is_starts_with(&code, "<", i) {
                let j = index_of(&code, ">", i+1);
                if j == -1 || i + 1 == j as usize || j as usize - i - 1 > 9 {
                    return false;
                }
                if code[i+1..j as usize].chars().filter(|ch| !ch.is_ascii_uppercase()).count() > 0 {
                    return false;
                }
                queue.push_back(&code[i + 1..j as usize]);
                i = j as usize + 1;
            } else {
                i += 1;
            }
        }
        queue.is_empty()
    }
}

pub fn is_starts_with(code: &str, target: &str, i: usize) -> bool {
    code[i..].starts_with(target)
}

pub fn index_of(code: &str, target: &str, i: usize) -> i32 {
    if let Some(j) = code[i..].find(target) { (i + j) as i32 } else { -1 }
}



impl Solution {
    fn parse_tag_name(input: &str) -> Option<(&str, &str)> {
        let search_area = if input.len() <= 9 { input } else { &input[..9] };

        let length = search_area
            .bytes()
            .position(|c| !c.is_ascii_uppercase())
            .unwrap_or_else(|| search_area.len());

        if length == 0 {
            None
        } else {
            Some(input.split_at(length))
        }
    }

    fn parse_cdata(input: &str) -> Option<&str> {
        let input = input.strip_prefix("<![CDATA[")?;
        let length = input.find("]]>")?;

        Some(&input[length..])
    }

    fn parse_text(input: &str) -> Option<&str> {
        let length = input.bytes().position(|c| c == b'<').unwrap_or_else(|| input.len());

        if length == 0 {
            None
        } else {
            Some(&input[length..])
        }
    }

    fn parse_content(mut input: &str) -> &str {
        while let Some(next_input) = Self::parse_tag(input)
            .or_else(|| Self::parse_cdata(input))
            .or_else(|| Self::parse_text(input))
        {
            input = next_input;
        }

        input
    }

    fn parse_tag(input: &str) -> Option<&str> {
        let input = input.strip_prefix("<")?;
        let (tag_name, input) = Self::parse_tag_name(input)?;
        let input = input.strip_prefix(">")?;
        let input = Self::parse_content(input);
        let input = input.strip_prefix("</")?;
        let input = input.strip_prefix(tag_name)?;

        input.strip_prefix(">")
    }

    pub fn is_valid(code: String) -> bool {
        Self::parse_tag(&code) == Some("")
    }
}
// 作者：kyushu
// 链接：https://leetcode.cn/problems/tag-validator/solution/by-kyushu-f625/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。