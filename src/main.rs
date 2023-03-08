pub fn main() {
    let ans = can_complete_circuit(vec![1,2,3,4,5],  vec![3,4,5,1,2]);
    assert_eq!(ans, 3);
}

pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut cur = 0;
    let (mut sum, mut pre) = (0, 0);
    let retain: Vec<i32> = gas.iter().zip(cost).map(|(&x, y)| x - y).collect();

    for (i, &n) in retain.iter().enumerate() {
        sum += n;
        
        if sum < 0 {
            pre += sum;
            sum = 0;
            cur = i + 1;
        }
    }
    if pre + sum < 0 {
        -1
    } else {
        cur as i32
    }
}