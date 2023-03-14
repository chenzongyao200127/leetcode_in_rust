// 424_Longest_Repeating_Character_Replacement
// https://leetcode.cn/problems/longest-repeating-character-replacement/

/// https://leetcode.cn/problems/longest-repeating-character-replacement/solution/ti-huan-hou-de-zui-chang-zhong-fu-zi-fu-eaacp/
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let len = s.len();
        if len < 2 {
            return len as i32;
        }


        let mut map = vec![0; 26];
        let s: Vec<char> = s.chars().collect();
        let mut ans = k; 
        let mut left = 0;
        let mut right = 0;

        while right < len {
            map[s[right] as usize - 'A' as usize] += 1;
            right += 1;

            if !is_map_ok(&map, k as usize) {
                map[s[left] as usize - 'A' as usize] -= 1;
                left += 1;
            }
            ans = ans.max((right - left) as i32);
        }

        ans
    }
}


pub fn is_map_ok(map: &Vec<usize>, k: usize) -> bool {
    let mut max = 0;
    for num in map {
        max = max.max(*num);
    }
    let diff = map.iter().sum::<usize>() - max;
    if diff <= k {
        true
    } else {
        false
    }
} 




use std::cmp::max;

macro_rules! pos {
    ($e:expr) => {
        ($e-b'A') as usize
    };
}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let n = s.len();
        let k = k as usize;
        if k+1 >= n {
            return n as i32;
        }
        let mut count = [0; 26];
        let (mut left, mut right) = (0,0);
        let mut ans = 1;
        let mut cur_max = 0;
        let s = s.as_bytes();
        while right < n {
            count[pos!(s[right])] += 1;
            cur_max = max(cur_max, count[pos!(s[right])]);
            while right-left+1>cur_max+k {
                count[pos!(s[left])] -= 1;
                left+=1;
            }
            ans = max(ans, right-left+1);
            right+=1;
        }
        ans as i32
    }
}


// class Solution:
//     def characterReplacement(self, s: str, k: int) -> int:
//         n = len(s)
//         res = 0
//         right = 0
//         left = 0
//         maxCnt = 0
//         window = [0] * 26

//         while right < n:
//             char_right = ord(s[right]) - ord('A')
//             window[char_right] += 1
//             maxCnt = max(maxCnt, window[char_right])

//             # 维护一个maxCnt+k大小的窗口，因为只能替换k次
//             if right - left + 1 > maxCnt + k:
//                 char_left = ord(s[left]) - ord('A')
//                 window[char_left] -= 1
//                 left += 1
//                 # 为何不更新maxCnt：我们始终维护一个大小为maxCnt + k的窗口；
//                 # 能得到的最大长度一定是在窗口内，只有当窗口变大时，结果才会更新，换句话说，当maxCnt变大时，才会变化
//                 # 对于下一个进窗口的字母，如果跟maxCnt的一样，那不需要移动左指针；
//                 # 如果不一样，那么移动左指针，但此时left变大，并不会是的res改变，所以无需更新maxCnt

//             res = max(res, right - left + 1)
//             right += 1

//         return res