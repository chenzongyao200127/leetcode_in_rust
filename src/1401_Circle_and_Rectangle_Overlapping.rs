// 1401_Circle_and_Rectangle_Overlapping
// https://leetcode.cn/problems/circle-and-rectangle-overlapping/

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let get_val = |center: i32, v1: i32, v2: i32| -> i32 {
            let min = v1.min(v2);
            let max = v1.max(v2);
            if min <= center && center <= max {
                return center;
            }
            if (center - min).abs() > (center - max).abs() {
                return max;
            }
            min
        };
        let x = get_val(x_center, x1, x2);
        let y = get_val(y_center, y1, y2);
        (x - x_center).pow(2) + (y - y_center).pow(2) <= radius * radius
    }
}



impl Solution {
    pub fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let x_closest = x_center.clamp(x1, x2);
        let y_closest = y_center.clamp(y1, y2);

        let dx = x_center - x_closest;
        let dy = y_center - y_closest;

        dx * dx + dy * dy <= radius * radius
    }
}