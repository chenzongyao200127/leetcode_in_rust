
pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
    let mut s = String::new();
    let mut ans = 0;
    for _ in 0..flips.len() {
        s.push('0');
    }
    let mut s = s.chars().collect::<Vec<char>>();
    for i in 0..flips.len() { 
        s[flips[i] as usize - 1] = '1';
        if is_prefix_aligned(&s, i+1) {
            ans += 1;
        }
    }
    ans
}

pub fn is_prefix_aligned(s: &Vec<char>, idx: usize) -> bool {
    let mut s_tmp = String::new();
    for _ in 0..idx { 
        s_tmp.push('1');
    }
    for _ in idx..s.len() {
        s_tmp.push('0');
    }
    &s_tmp.chars().collect::<Vec<char>>() == s
}

pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut ans = vec![];
    for querie in queries {
        let left = querie[0] as usize;
        let right = querie[1] as usize;
        let k = querie[2];
        if k as usize >= helper(&s[left as usize..right as usize+1]) {
            ans.push(true);
        } else {
            ans.push(false);
        }
    }

    ans
}


pub fn helper(s: &str) -> usize {
    let len = s.len();
    let s1 = s[0..len/2].to_string().chars().collect::<Vec<char>>();
    let mut s2 = s[len/2..len].to_string().chars().collect::<Vec<char>>();
    if len % 2 == 1 { 
        s2.remove(0);
    }
    let mut diff = 0;
    for i in 0..s.len()/2 {
        if s1[i] != s2[s2.len()-1-i] {
            diff += 1;
        }
    }
    diff
}

fn main() {
    println!("{:?}", can_make_pali_queries("abcda".to_string(), vec![vec![3,3,0],vec![1,2,0],vec![0,3,1],vec![0,3,2],vec![0,4,1]]));
}
