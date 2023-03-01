// 150. Evaluate Reverse Polish Notation
// https://leetcode.cn/problems/evaluate-reverse-polish-notation/

// 4 ms 45.45%
// 2.7 MB 27.27%
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stk: Vec<i32> = vec![];
        tokens.iter().for_each(|s| {
            if let Ok(x) = s.parse::<i32>() {
                stk.push(x);
            } else {
                let b = stk.pop().unwrap();
                let a = stk.pop().unwrap();
                stk.push({
                    match s.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        _   => a / b,
                    }
                })
            } 
        });

        stk[0]
    }
}


// Other's Solution
impl Solution {
    pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        let mut v = Vec::with_capacity(10);
        tokens.drain(..).for_each(|x|{
            match x.as_str(){
                "+"=>{let c=v.pop().unwrap();*v.last_mut().unwrap()+=c},
                "-"=>{let c=v.pop().unwrap();*v.last_mut().unwrap()-=c},
                "*"=>{let c=v.pop().unwrap();*v.last_mut().unwrap()*=c},
                "/"=>{let c=v.pop().unwrap();*v.last_mut().unwrap()/=c},
                x=>{v.push(x.parse().unwrap())}
            }
        });
        v.last().copied().unwrap()
    }
}
// 作者：qweytr_1
// 链接：https://leetcode.cn/problems/evaluate-reverse-polish-notation/solution/rust-wei-shi-yao-da-jia-bu-yong-last_mut-hbfd/