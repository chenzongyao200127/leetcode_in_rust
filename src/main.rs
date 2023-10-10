fn main() {
    let ans = h_index(vec![0]);
    println!("{:?}", ans);
}

pub fn h_index(citations: Vec<i32>) -> i32 {
    #[inline]
    fn check(idx: usize, citations: &Vec<i32>) -> i32 {
        let h_cit = citations[idx] as usize;
        let nums = citations.len() - idx;
        if h_cit > nums {
            return 1;
        } else if h_cit < nums {
            return -1;
        } else {
            return 0;
        }
    }

    if citations.is_empty() {
        return 0;
    }

    let mut l = 0;
    let mut r = citations.len() - 1;
    let mut mid;

    while l < r {
        mid = l + (r - l) / 2;
        match check(mid, &citations) {
            1 => r = if mid == 0 { 0 } else { mid - 1 },
            -1 => l = mid + 1,
            _ => return (citations.len() - mid) as i32,
        }
    }

    (citations.len() - l) as i32
}