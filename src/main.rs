pub fn main() {
    // let ans = best_team_score(vec![4,6,5,7,8,2], vec![3,2,4,6,5,2]);
    // assert_eq!(ans, 19);

    // let ans = best_team_score(vec![4,5,6,5], vec![2,1,2,1]);
    // assert_eq!(ans, 16);

    let ans = check_arithmetic_subarrays(vec![-12,-9,-3,-12,-6,15,20,-25,-20,-15,-10], vec![0,1,6,4,8,7], vec![4,4,9,7,9,10]);
    assert_eq!(ans, vec![false,true,false,false,true,true]);

}

pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    let mut ans = vec![];
    for (l, r) in l.into_iter().zip(r) {
        ans.push(check_arithmetic_array(&nums[l as usize..r as usize+1].to_vec()));
    }

    ans
}

pub fn check_arithmetic_array(nums: &Vec<i32>) -> bool {
    let mut nums = nums.clone();
    nums.sort_unstable();

    let diff = nums[1] - nums[0];
    for i in 1..nums.len()-1 {
        if nums[i+1] - nums[i] == diff {
            continue;
        } else {
            return false;
        }
    }
    
    true
}
