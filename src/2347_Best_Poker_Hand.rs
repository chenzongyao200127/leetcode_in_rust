// 2347. Best Poker Hand

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        for i in 0..suits.len() {
            if suits[i] == suits[0] {
                if i == 4 {
                    return "Flush".to_string();
                } else {
                    continue;
                }
            } else {
                break;
            }
        }
    
        let mut max_same_nums = 1;
        for i in 0..ranks.len()-1 {
            let mut tmp = 1;
            for j in 0..ranks.len() {
                if ranks[i] == ranks[j] {
                    tmp += 1;
                }
            }
            max_same_nums = max_same_nums.max(tmp - 1);
        }
    
        match max_same_nums {
            1 => return "High Card".to_string(),
            2 => return "Pair".to_string(),
            _ => return "Three of a Kind".to_string()
        }
    }
}


impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        match suits.iter().collect::<std::collections::HashSet<_>>().len() == 1 {
            true => "Flush".to_string(),
            false => {
                let cnt = ranks.iter().fold([0;14], |mut acc ,&r| {
                    acc[r as usize] += 1;
                    acc
                });
                match cnt.into_iter().max() {
                    Some(1) => "High Card".to_string(),
                    Some(2) => "Pair".to_string(),
                    _ => "Three of a Kind".to_string(),
                }
            }
        }
    }
}
// 作者：DrackRamoray
// 链接：https://leetcode.cn/problems/best-poker-hand/solution/rust-by-drackramoray-lsxc/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。


impl Solution {
    pub fn best_hand(mut ranks: Vec<i32>, suits: Vec<char>) -> String {
        if suits.iter().all(|&x|x==suits[0]) {
            "Flush"
        } else {
            ranks.sort_unstable();
            let c=ranks.windows(2).fold((0,0),|mut s,x|{if x[0]==x[1] {s.1+=1} else {s.0=s.0.max(s.1);s.1=0};s});
            match c.0.max(c.1) {
                0=>"High Card",
                1=>"Pair"
                ,_=>"Three of a Kind"
            }
        }.to_string()
    }
}
// 作者：qweytr_1
// 链接：https://leetcode.cn/problems/best-poker-hand/solution/rust-windowsde-miao-yong-by-qweytr_1-17ms/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。