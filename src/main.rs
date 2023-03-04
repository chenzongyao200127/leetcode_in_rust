use std::collections::{HashMap, HashSet};

fn main() {
    // let ans = get_max_repetitions("abc".to_string(), 4, "ab".to_string(), 2);
    // assert_eq!(ans, 2);

    // let ans = candy(vec![1,2,87,87,87,2,1]);
    // assert_eq!(ans, 13);
    sort_colors(&mut vec![1,2,0,0,0,2,1,1,2,2,1,0,2,1]);
}


pub fn sort_colors(nums: &mut Vec<i32>) {
    let (mut num0, mut num1, mut num2) = (0, 0, 0);
    for i in 0..nums.len() {
        if nums[i] == 0 {
            nums[num2] = 2;
            nums[num1] = 1;
            nums[num0] = 0;
            num0 += 1;
            num1 += 1;
            num2 += 1;
        }else if nums[i] == 1  {
            nums[num2] = 2;
            nums[num1] = 1;
            num1 += 1;
            num2 += 1;
        }else {
            nums[num2] = 2;
            num2 += 1;
        }
        println!("{:?}", nums);
    }
}
