pub fn main() {
    let ans = reverse_vowels("hello".to_string());
    assert_eq!(ans, "holle".to_owned());
}


// pub fn main() {
//     let ans = can_complete_circuit(vec![3,3,4], vec![3,4,4]);
//     assert_eq!(ans, -1);
// }

// pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
//     let mut diff = vec![];
//     for i in 0..gas.len() {
//         diff.push(gas[i] - cost[i]);
//     }
//     let mut diff: Vec<_> = diff.iter().enumerate().collect();
//     diff.sort_unstable_by(|a, b| (b.1).cmp(&(a.1)));
//     let start = diff[0].0;
//     let mut fuel = gas[start];
//     let mut cur = start;
//     println!("{:?}", fuel);
//     while cur < gas.len()-1 {
//         fuel  -= cost[start];
//         if fuel < 0 {
//             return -1;
//         }
//         println!("{:?}", fuel);
//         cur += 1;
//         fuel += gas[cur];
//         println!("{:?}", fuel);
//     }
//     cur = 0;
//     while cur <= start {
//         if cur == 0 {
//             fuel -= cost[gas.len()-1];
//         } else {
//             fuel -= cost[cur - 1];
//         }
//         println!("{:?}", fuel);
//         if fuel < 0 {
//             return -1;
//         }
//         if cur == start {
//             break;
//         }
//         fuel += gas[cur];
//         println!("{:?}", fuel);
//         cur += 1;
//     }

//     start as i32
// }