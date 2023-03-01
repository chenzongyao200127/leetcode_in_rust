// 1255. Maximum Score Words Formed by Letters
// https://leetcode.cn/problems/maximum-score-words-formed-by-letters/


// 方法一：状态压缩
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut s_set = 1 as i32;
        let len = words.len();
        let mut letters_cnt = vec![0; 26];
        // let mut words_cnt = vec![0; 26];
        let mut ans = 0;
        letters.iter().for_each(|c| {
            letters_cnt[((*c) as u8 - 'a' as u8) as usize] += 1;
        });
        while s_set < (1 << len)  {
            let mut s_letters_cnt = vec![0; 26];
            for k in 0..len {
                if s_set & (1 << k) == 0 {
                    continue;
                }
                words[k].chars().for_each(|c| {
                    s_letters_cnt[((c) as u8 - 'a' as u8) as usize] += 1;
                });
                let mut ok = true;
                let mut sum = 0;
                for i in 0..26 {
                    sum += s_letters_cnt[i] * score[i];
                    ok = ok && s_letters_cnt[i] <= letters_cnt[i];
                }
                if ok { ans = ans.max(sum); }
            } 
    
            s_set += 1;
        }
    
        ans
    }
}


impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut letter_num = 0;
        let mut cnt = [0; 26];
        for &ch in &letters {
            letter_num |= 1 << ch as u8 - b'a';
            cnt[(ch as u8 - b'a') as usize] += 1;
        }
        let words: Vec<[u8; 26]> = words.into_iter().filter_map(|s| {
            let mut num = 0;
            let mut cnt = [0; 26];
            for &ch in s.as_bytes() {
                num |= 1 << ch - b'a';
                cnt[(ch - b'a') as usize] += 1;
            }
            if num & letter_num == num {
                Some(cnt)
            } else { None }
        }).collect();
    
        fn dfs(words: &Vec<[u8; 26]>, ori_cnt: &[u8], score: &Vec<i32>, i: usize, cnt: [u8; 26], result: &mut i32) {
            if i == words.len() {
                (*result) = (*result).max((0..26).map(|x| {
                    score[x] * (ori_cnt[x] - cnt[x]) as i32
                }).sum());
                return;
            }
            let mut ok = true;
            let wcnt = &words[i];
            let mut next_cnt = cnt.clone();
            for i in 0..26 {
                if cnt[i] < wcnt[i] {
                    ok = false;
                    break;
                }
                next_cnt[i] = cnt[i] - wcnt[i];
            }
            if ok {
                dfs(words, ori_cnt, score, i + 1, next_cnt, result);
            }
            dfs(words, ori_cnt, score, i + 1, cnt, result);
        }
        let mut result = 0;
        dfs(&words, &cnt, &score, 0, cnt.clone(), &mut result);
        result
    }
    
}

// 子集型回溯
