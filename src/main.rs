fn main() {
    // let s1 = "01000111";
    // let s2 = "00111";
    // let s3 = "111";

    // println!("Example 1: {}", find_the_longest_balanced_substring(s1.to_string())); // Output: 6
    // println!("Example 2: {}", find_the_longest_balanced_substring(s2.to_string())); // Output: 4
    // println!("Example 3: {}", find_the_longest_balanced_substring(s3.to_string())); // Output: 0

    let ans = prev_perm_opt1(vec![1,9,4,7,6,7]);
    assert_eq!(ans, vec![1,9,4,6,7,7]);

    let ans = prev_perm_opt1(vec![1,1,7]);
    assert_eq!(ans, vec![1,1,7]);

    let ans = prev_perm_opt1(vec![3,2,1]);
    assert_eq!(ans, vec![3,1,2]);

    let ans = prev_perm_opt1(vec![3,1,1,3]);
    assert_eq!(ans, vec![1,3,1,3])
}

pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    for i in (0..arr.len()-1).rev() {
        if arr[i] > arr[i+1] {
            let mut tmp_max = (arr[i+1], i+1);
            for j in i+1..arr.len() {
                if arr[j] > tmp_max.0 && arr[j] < arr[i] {
                    tmp_max = (arr[j], j);
                }
            }

            let tmp = arr[i];
            arr[i] = tmp_max.0;
            arr[tmp_max.1] = tmp;
            break;
        }
    }

    return arr;
}


// pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
//     let mut arr = arr;
//     let mut idx = 0;
//     while idx < arr.len() {
//         if arr[idx] != 1 {
//             break;
//         }
//         idx += 1;
//     }

//     if idx == arr.len() - 1 {
//         return arr;
//     }

//     for left in idx..arr.len()-1 {
//         let mut tmp = (0, arr.len()+1);
//         for right in ((left+1)..arr.len()).rev() {
//             if arr[right] < arr[left] && arr[right] >= tmp.0 && right <= tmp.1 {
//                 tmp = (arr[right], right);
//             }
//         }
//         if tmp != (0, arr.len()+1) {
//             let t = arr[left];
//             arr[left] = tmp.0;
//             arr[tmp.1] = t;
//             break;
//         }

//     }

//     return arr
// }