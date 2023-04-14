// 1023. Camelcase Matching
// https://leetcode.cn/problems/camelcase-matching/

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        fn is_match(query: &str, pattern: &str) -> bool {
            let (mut i, query_arr, pattern_arr) = (0, query.as_bytes(), pattern.as_bytes());
            for &ch in query_arr {
                if i < pattern_arr.len() && ch == pattern_arr[i] { i += 1; } else if ch < b'a' { return false; }
            }
            i == pattern.len()
        }

        queries.into_iter().map(|query| is_match(query.as_str(), pattern.as_str())).collect::<Vec<_>>()
    }
}


fn tmatch(q: &str, pat: &str) -> bool {
    let q = q.as_bytes();
    let pat = pat.as_bytes();
    let mut i = 0;
    for x in pat {
        if i >= q.len() {
            return false;
        }
        let mut tm = false;
        while i < q.len() {
            //println!("i {}, x {}",i,*x as char);
            if q[i] == *x {
                tm = true;
                break;
            }else if q[i].is_ascii_uppercase() {
                return false;
            }else {
                i += 1;
            }
        }
        if !tm {
            return false;
        }
        i += 1;
    }
    while i < q.len() {
        if q[i].is_ascii_uppercase() {
            return false;
        }
        i += 1;
    }
    true
}

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        queries.iter().map(|s| tmatch(s, &pattern)).collect()
    }
}


// class Solution {
//     public List<Boolean> camelMatch(String[] queries, String pattern) {
//         String newPattern = "[a-z]*" + String.join("[a-z]*", pattern.split("")) + "[a-z]*";
//         return Arrays.stream(queries).map(query -> query.matches(newPattern)).collect(Collectors.toList());
//     }
// }
