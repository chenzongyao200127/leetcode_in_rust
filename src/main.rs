pub fn main() {
    let ans = merge(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]]);
    assert_eq!(ans, vec![vec![1,6],vec![8,10],vec![15,18]]);
}

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
    println!("{:?}", intervals);
    let mut left = intervals[0][0];
    let mut right = intervals[0][1];
    let mut ans: Vec<Vec<i32>> = vec![];
    for i in 1..intervals.len() {
        if intervals[i][0] <= right {
            right = right.max(intervals[i][1]);
        } else {
            ans.push(vec![left, right]);
            left = intervals[i][0];
            right = intervals[i][1];
        }
    }
    ans.push(vec![left, right]);

    ans
}
