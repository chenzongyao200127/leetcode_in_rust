// 649. Dota2 Senate
// https://leetcode.cn/problems/dota2-senate/


use std::collections::VecDeque;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiant = VecDeque::new();
        let mut dire = VecDeque::new();
        senate.chars().enumerate().for_each(|(i, c)| {
            if c == 'R' {
                radiant.push_back(i);
            } else {
                dire.push_back(i)
            }
        });
        while !radiant.is_empty() && !dire.is_empty() {
            let (r, d) = (radiant.pop_front().unwrap(), dire.pop_front().unwrap());
            if r < d {
                radiant.push_back(r + senate.len());
            } else {
                dire.push_back(d + senate.len());
            }
        }

        if radiant.is_empty() {
            "Dire".into()
        } else {
            "Radiant".into()
        }
    }
}


// class Solution:
//     def predictPartyVictory(self, senate: str) -> str:
//         n = len(senate)
//         radiant = collections.deque()
//         dire = collections.deque()
        
//         for i, ch in enumerate(senate):
//             if ch == "R":
//                 radiant.append(i)
//             else:
//                 dire.append(i)
        
//         while radiant and dire:
//             if radiant[0] < dire[0]:
//                 radiant.append(radiant[0] + n)
//             else:
//                 dire.append(dire[0] + n)
//             radiant.popleft()
//             dire.popleft()
        
//         return "Radiant" if radiant else "Dire"

// 作者：LeetCode-Solution
// 链接：https://leetcode.cn/problems/dota2-senate/solution/dota2-can-yi-yuan-by-leetcode-solution-jb7l/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。