// 2300_Successful_Pairs_of_Spells_and_Potions
// https://leetcode.cn/problems/successful-pairs-of-spells-and-potions/description/

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
    
        #[inline]
        fn search(potions: &Vec<i32>, spell: i32, success: i64) -> usize {
            let mut l = 0;
            let mut r = potions.len();
            while l < r {
                let mid = l + (r - l) / 2;
                if (potions[mid] as i64 * spell as i64) < success {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
    
            l
        }
    
        spells.iter().map(|&spell| (potions.len() - search(&potions, spell, success)) as i32).collect()
    }
}