pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut tmp = vec![];
    dfs(&mut tmp, 1, n, k, &mut ans);
    ans
}

fn dfs(tmp: &mut Vec<i32>, x: i32, n: i32, k: i32, ans: &mut Vec<Vec<i32>>) {
    if tmp.len() == k as usize {
        ans.push(tmp.clone());
        return;
    }

    if x > n {
        return;
    }

    dfs(tmp, x + 1, n, k, ans);

    tmp.push(x);
    dfs(tmp, x + 1, n, k, ans);
    tmp.pop();
}

pub fn maximum_sum(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut dp_keep = vec![i32::MIN; len];
    let mut dp_delete = vec![i32::MIN; len];
    dp_keep[0] = arr[0];
    dp_delete[0] = arr[0];
    for i in 1..len {
        dp_keep[i] = arr[i].max(dp_keep[i - 1] + arr[i]);
        dp_delete[i] = arr[i].max(dp_keep[i - 1].max(dp_delete[i - 1] + arr[i]));
    }
    return dp_delete.into_iter().max().unwrap()
}

fn main() {
    println!("{:?}", combine(5,2))
}