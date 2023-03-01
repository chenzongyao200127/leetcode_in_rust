use std::collections::{HashSet, HashMap};

fn main() {
    // let ans = get_max_repetitions("abc".to_string(), 4, "ab".to_string(), 2);
    // assert_eq!(ans, 2);

    let ans = find_restaurant(vec!["happy".to_string(),"sad".to_string(),"good".to_string()], vec!["sad".to_string(),"happy".to_string(),"good".to_string()]);
    assert_eq!(ans, vec!["sad".to_string(), "happy".to_string()]);
}

pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut map: HashMap<&str, usize> = HashMap::new();
    let mut ans_vec: Vec<(usize, usize)> = vec![];
    let mut ans = vec![];
    for i in 0..list2.len() {
        map.insert(&list2[i], i);
    }
    for i in 0..list1.len() {
        if let Some(&cnt) = map.get(&list1[i][..]) {
            ans_vec.push((i, cnt + i));
        }
    }
    ans_vec.sort_by_key(|x| x.1);
    let min = ans_vec[0].1;

    for (i, j) in ans_vec.into_iter() {
        if j == min {
            ans.push(list1[i].to_string());
        }
    }

    ans
}

pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
    let str1: Vec<char> = s1.chars().collect();
    let str2: Vec<char> = s2.chars().collect();
    let mut dp = vec![0; 100];
    let mut str2_cur = 0;
    let mut ans = 0;
    let mut cnt = 1;
    while cnt <= n1 {
        for i in 0..s1.len() {
            if str1[i] == str2[str2_cur] {
                str2_cur = (str2_cur+1) % str2.len();
                if str2_cur == 0 {
                    ans += 1;
                }
            }
        }
        dp[cnt as usize] = ans;
        if str2_cur == 0 {
            break;
        }
        cnt += 1;
    }

    return (n1/cnt as i32*ans + dp[n1 as usize%cnt as usize])as i32/n2 as i32;
}