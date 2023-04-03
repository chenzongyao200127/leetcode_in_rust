// 1053_Previous_Permutation_With_One_Swap
// https://leetcode.cn/problems/previous-permutation-with-one-swap/

/// 贪心
impl Solution {
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
                
                arr.swap(i, tmp_max.1);
                break;
            }
        }

        return arr;
    }
}


impl Solution {
    pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        
        for i in (0..arr.len()-1).rev() {
            if arr[i] > arr[i+1] {
                let mut tmp_max = i+1;

                for j in i+1..arr.len() {
                    if arr[j] > arr[tmp_max] && arr[j] < arr[i] {
                        tmp_max = j;
                    }
                }
                
                arr.swap(i, tmp_max);
                break;
            }
        }

        return arr;
    }
}