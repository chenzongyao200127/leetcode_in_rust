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


fn main() {
    println!("{:?}", combine(5,2))
}