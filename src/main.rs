fn main() {

}

pub fn average_value(nums: Vec<i32>) -> i32 {
    let nums: Vec<_> = nums.into_iter().filter(|&x| x%6==0).collect();
    if nums.len() == 0 {
        return 0;
    } else {
        nums.iter().sum::<i32>() / nums.len() as i32
    }
}