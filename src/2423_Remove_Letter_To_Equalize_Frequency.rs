// 2423_Remove_Letter_To_Equalize_Frequency
// https://leetcode.cn/problems/remove-letter-to-equalize-frequency/


impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        fn is_equal(cnt: &Vec<i32>) -> bool {
            let mut last = 0;
            for &c in cnt {
                if c == 0 { continue; }
                if last != 0 && last != c { return false; }
                last = c;
            }
            true
        }

        let mut cnt = vec![0; 26];
        word.bytes().for_each(|ch| cnt[(ch - b'a') as usize] += 1);
        for i in 0..26 {
            if cnt[i] == 0 { continue; }
            cnt[i] -= 1;
            if is_equal(&cnt) { return true; }
            cnt[i] += 1;
        }
        false
    }
}