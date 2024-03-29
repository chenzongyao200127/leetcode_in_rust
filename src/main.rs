pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut max_area = 0;

    while left < right {
        let width = (right - left) as i32;
        let (left_height, right_height) = (height[left], height[right]);

        max_area = max_area.max(width * left_height.min(right_height));

        if left_height < right_height {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

fn main() {
    println!("Hello World");
}
