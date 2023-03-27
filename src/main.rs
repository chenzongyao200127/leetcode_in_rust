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


pub fn count_substrings(s: String, t: String) -> i32 {
    let mut dpl = vec![vec![0; t.len()]; s.len()];
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    for i in 1..s.len() {
        for j in 1..t.len() {
            if s[i] == s[j] {
                dpl[i][j] = dpl[i-1][j-1] + 1;
            } else {
                dpl[i][j] = 0;
            }
        }
    }
    let mut dpr = vec![vec![0; t.len()]; s.len()];
    for i in (0..s.len()-1).rev() {
        for j in (0..t.len()-1).rev() {
            if s[i] == s[j] {
                dpr[i][j] = dpr[i+1][j+1] + 1;
            } else {
                dpr[i][j] = 0;
            }
        }
    }
    let mut ans = 0;
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] != t[j] {
                ans += (dpl[i][j] + 1) * (dpr[i][j] + 1);
            }
        }
    }
    ans
}