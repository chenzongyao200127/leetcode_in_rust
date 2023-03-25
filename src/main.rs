pub fn main() {
    let ans = find_length_of_shortest_subarray(vec![5,4,3,2,1]);
    assert_eq!(ans, 4);

    let ans = find_length_of_shortest_subarray(vec![1,2,3]);
    assert_eq!(ans, 0);

    let ans = find_length_of_shortest_subarray(vec![1,2,3,10,0,7,8,9]);
    assert_eq!(ans, 2);

    let ans = find_length_of_shortest_subarray(vec![13,0,14,7,18,18,18,16,8,15,20]);
    assert_eq!(ans, 8);
}


pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut i = 0;
    let mut j = n - 1;

    while i < n - 1 && arr[i] <= arr[i + 1] {
        i += 1;
    }
    while j > 0 && arr[j-1] <= arr[j] {
        j -= 1;
    }
    if i >= j {
        return 0;
    }

    let mut ans = j.min(n-i-1);
    let mut right = j;
    for left in 0..i+1 {
        while right < n && arr[right] < arr[left] {
            right += 1;
        }
        ans = ans.min(right - left - 1);
    }
    
    ans as i32
}