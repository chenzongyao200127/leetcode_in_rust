
fn main() {
    let ans = max_sum_after_partitioning(vec![1,15,7,9,2,5,10], 3);
    assert_eq!(ans, 84)
}


pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    let mut dp = vec![0; arr.len()];
    
}


// pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
//     let len = arr.len();
//     let remainder = len as i32 % k;
//     let mut ans = 0;

//     for i in 0..=remainder as usize {
//         let mut tmp_vecs : Vec<Vec<i32>> = Vec::new();
//         tmp_vecs.push(arr[0..i].to_vec());
//         for j in (i..arr.len() - (remainder as usize - i)).step_by(k as usize) {
//             tmp_vecs.push(arr[j..j+k as usize].to_vec());
//         }
//         tmp_vecs.push(arr[len-remainder as usize+i..len].to_vec());
//         println!("{:?}", tmp_vecs);

//         let mut tmp = 0;
//         for vec in tmp_vecs {
//             let len = vec.len();
//             if len != 0 {
//                 let &max_in_vec = vec.iter().max().unwrap();
//                 tmp += max_in_vec  * len as i32;
//             }
//         }
//         ans = ans.max(tmp as i32);
//     }

//     ans
// }