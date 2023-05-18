pub fn find_max_k(nums: Vec<i32>) -> i32 {
    let mut filter_nums: Vec<_> = nums.iter().filter(|&x| nums.contains(&(-x))).collect();
    filter_nums.sort_unstable();
    if filter_nums.len() == 0 { return -1 } else { (*filter_nums[0]).abs() }
}

fn main() {
    let numbers = vec![-1,10,6,7,-7,1];
    let result = find_max_k(numbers);
    println!("Answer: {:?}", result);
}