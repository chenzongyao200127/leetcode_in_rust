pub fn main() {
    let ans = reconstruct_queue(vec![vec![7,0],vec![4,4],vec![7,1],vec![5,0],vec![6,1],vec![5,2]]);
    assert_eq!(ans, vec![vec![5,0],vec![7,0],vec![5,2],vec![6,1],vec![4,4],vec![7,1]]);
}

pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut people = people;
    let mut ans = vec![];
    people.sort_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));
    for p in people.iter() {
        ans.insert(p[1] as usize, p.to_vec());
    }
    ans
}