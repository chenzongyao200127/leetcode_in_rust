
pub fn main() {
    let ans = trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]);
    assert_eq!(ans, 6);
}

pub fn trap(height: Vec<i32>) -> i32 {
    let mut stk: Vec<usize>= vec![];
    let mut ans = 0;

    for i in 0..height.len() {
        while !stk.is_empty() && height[stk[stk.len()-1]] < height[i] {
            let top = stk.pop().unwrap();
            if stk.is_empty() {
                break;
            }
            let left = stk[stk.len()-1];
            ans += (i-left-1)*((height[i].min(height[left])-height[top]) as usize);
        }
        stk.push(i);
    }

    ans as i32
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    nums.binary_search(&target).unwrap_or_else(|x| x) as i32
}