
pub fn main() {
    let ans = remove_kdigits("112".to_string(), 1);
    assert_eq!("11".to_string(), ans);
}

// pub fn remove_kdigits(num: String, k: i32) -> String {
//     if k as usize == num.len() {
//         return "0".to_string();
//     }
//     let mut s: Vec<char> = num.chars().collect();
//     for _ in 0..k {
//         for i in 1..s.len() {
//             let pre = s[i-1] as u8;
//             if pre <= s[i] as u8 {
//                 if i == s.len()-1 {
//                     s.remove(i);
//                 } else {
//                     continue;
//                 }
//             } else {
//                 s.remove(i-1);
//                 break;
//             }
//         }
//     }

//     let mut ans: String = s.iter().collect();
//     if ans == "0".repeat(num.len() - k as usize) {
//         return "0".to_string();
//     }

// }