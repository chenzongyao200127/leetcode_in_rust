// 2490_Circular_Sentence
// https://leetcode.cn/problems/circular-sentence/

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words = sentence.split(' ').collect::<Vec<&str>>();
        if sentence.starts_with(' ') || sentence.ends_with(' ') {
            return false;
        }
        if sentence.chars().into_iter().filter(|ch| !(ch.is_alphabetic() || ch.is_whitespace())).count() != 0 {
            return  false;
        }
        for i in 0..words.len() {
            if &words[i][words[i].len()-1..words[i].len()] != &words[(i+1+words.len())%words.len()][0..1] {
                return false;
            }
        }
        true
    }
}


impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let ss: Vec<String> = sentence.split(' ').map(String::from).collect();
        let n = ss.len();
        for i in 0..n {
            if ss[i].as_bytes()[ss[i].len() - 1] != ss[(i + 1) % n].as_bytes()[0] {
                return false;
            }
        }
        return true;
    }
}