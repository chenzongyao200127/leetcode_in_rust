pub fn main() {
    let ans = max_width_of_vertical_area(vec![vec![3,1],vec![9,0],vec![1,0],vec![1,4],vec![5,3],vec![8,8]]);
    assert_eq!(ans, 3);
}


pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut tmp = vec![];
    for vec in points {
        tmp.push(vec[0]);
    }
    tmp.sort_unstable();
    let mut ans = 0;
    for i in 1..tmp.len() {
        ans = ans.max(tmp[i] - tmp[i-1]);
    }
    ans
}