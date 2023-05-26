// 17_Letter_Combinations_of_a_Phone_Number
// https://leetcode.cn/problems/letter-combinations-of-a-phone-number/

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map: std::collections::HashMap<char, &str> = vec![
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ].into_iter().collect();

        let strs = digits.chars().map(
            |c| map.get(&c).unwrap()
        );

        let mut results: Vec<String> = vec![];

        for s in strs {
            let new_results = s.chars().map(
                |c| {
                    if results.is_empty() {
                        vec![c.to_string()]
                    } else {
                        results.iter().cloned().map(
                        |mut s| {
                            s.push(c);
                            s
                        }).collect()
                    }
                }
            ).flatten().collect();

            results = new_results;            
        }

        results
    }
}