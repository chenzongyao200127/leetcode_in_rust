pub fn pivot_integer(n: i32) -> i32 {
    let mut pre_sum = 0 as i32;
    let mut suf_sum = n as i32;
    for i in 1..=n {
        pre_sum += i as i32;
    }
    for i in (1..=n-1).rev() {
        pre_sum -= i as i32 + 1;
        suf_sum += i as i32;
        if pre_sum == suf_sum {
            return i as i32;
        }
    }
    -1
}

fn main() {
    println!("{:?}", pivot_integer(8))
}