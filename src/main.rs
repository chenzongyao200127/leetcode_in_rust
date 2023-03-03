use std::collections::HashMap;

fn main() {
    // let ans = get_max_repetitions("abc".to_string(), 4, "ab".to_string(), 2);
    // assert_eq!(ans, 2);

    // let ans = candy(vec![1,2,87,87,87,2,1]);
    // assert_eq!(ans, 13);
    let ans = find_min_arrow_shots(vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5]]);
    assert_eq!(ans, 2);
}

pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut points = points;
    points.sort_unstable_by(|x, y| x[0].cmp(&y[0]));
    let mut start = i32::MIN;
    let mut end = i32::MAX;
    for point in points {
        if point[0] <= end {
            start = start.max(point[0]);
            end = end.min(point[1]);
        } else {
            start = point[0];
            end = point[1];
            ans += 1;
        }
        // println!("{:?}", (start, end));
    }

    ans+1
}