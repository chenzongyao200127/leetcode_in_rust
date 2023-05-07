
fn main() {
    let ans = num_pairs_divisible_by60(vec![60,60,60]);
    assert_eq!(ans, 3)
}

pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i64 {
    let time: Vec<i32> = time.into_iter().map(|x| x % 60).collect();

    let mut res = 0 as i64;

    let mut cnt = vec![0;60];
    time.iter().for_each(|&x| cnt[x as usize] += 1);

    for i in 1..30 {
        res += cnt[i] * cnt[60-i]
    }

    res += cnt[0] * (cnt[0] - 1) / 2;
    res += cnt[30] * (cnt[30] - 1) / 2;

    res
}