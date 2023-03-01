// 394. Decode String
// https://leetcode.cn/problems/decode-string/

impl Solution {
    // Didn't Work, So many bugs /(ㄒoㄒ)/~~
    // pub fn decode_string(s: String) -> String {
    //     let data: Vec<char> = s.chars().collect();
    //     let mut idx = 0;
    //     let mut num_stk: Vec<i32> = vec![];
    //     let mut str_stk: Vec<&str> = vec![];
    
    //     while idx < data.len()-1 {
    //         let mut offset = 1;
    
    //         if data[idx].is_ascii_digit() {
    //             while data[idx+offset].is_ascii_digit(){
    //                 offset += 1;
    //             }
    //             num_stk.push((&s[idx..idx+offset]).parse::<i32>().unwrap());
    //             idx = idx + offset;
    //             continue;
    //         } 
    //         if data[idx] == '[' {
    //             idx += 1;
    //             while !data[idx+offset].is_ascii_digit() && data[idx+offset] != '[' && data[idx+offset] != ']' && idx+offset < data.len()-1 {
    //                 offset += 1;
    //             }
    //             str_stk.push(&s[idx..idx+offset]);
    //             idx = idx + offset;
    //             continue;
    //         } 
    //         if data[idx] == ']' {
    //             str_stk[str_stk.len()-2].to_string().push_str(&str_stk[str_stk.len()-1].repeat(num_stk[num_stk.len()-1] as usize));
    //             str_stk.pop();
    //             num_stk.pop();
    //             continue;
    //         }
    //     }
    
    //     str_stk[0].to_string()
    // }

    // 0 ms 100%
    // 2 MB 53.58%
    pub fn decode_string(s: String) -> String {
        let mut stk = vec![];
        let mut num = "".to_string();
        
        s.chars().for_each(|c| match c {
            '0'..='9' => num.push(c),
            '[' => {
                stk.push(num.clone());
                stk.push(c.to_string());
                num = "".to_string();
            }
            ']' => {
                let mut tmp = vec![];
                while let Some(s) = stk.pop() {
                    if s == '['.to_string() {
                        break;
                    }
                    tmp.push(s);
                }
                tmp.reverse();
                let num = stk.pop().unwrap().parse().unwrap();
                stk.push(
                    tmp.iter()
                    .fold("".to_string(), |acc, s| acc + s)
                    .repeat(num)
                );
            }
            _ => stk.push(c.to_string()),
        });
    
        stk.iter().fold("".to_string(), |acc, s| acc + s)
    }
}

// Java
// class Solution {
//     public String decodeString(String s) {
            
//             Stack<Character> stack = new Stack<>();
            
//             for(char c : s.toCharArray())
//             {
//                 if(c != ']') 
//                     stack.push(c); // 把所有的字母push进去，除了]
                
//                 else 
//                 {
//                     //step 1: 取出[] 内的字符串
                    
//                     StringBuilder sb = new StringBuilder();
//                     while(!stack.isEmpty() && Character.isLetter(stack.peek()))
//                         sb.insert(0, stack.pop());
                    
//                     String sub = sb.toString(); //[ ]内的字符串
//                     stack.pop(); // 去除[
                    
                    
//                     //step 2: 获取倍数数字
                        
//                     sb = new StringBuilder();
//                     while(!stack.isEmpty() && Character.isDigit(stack.peek()))
//                         sb.insert(0, stack.pop());
                        
//                     int count = Integer.valueOf(sb.toString()); //倍数
                    
                    
//                     //step 3: 根据倍数把字母再push回去
                    
//                     while(count > 0)
//                     {
//                         for(char ch : sub.toCharArray())  
//                             stack.push(ch);
//                         count--;
//                     }
//                 }
//             }
            
//           //把栈里面所有的字母取出来，完事L('ω')┘三└('ω')｣
//             StringBuilder retv = new StringBuilder();
//             while(!stack.isEmpty())
//                 retv.insert(0, stack.pop());
    
//             return retv.toString();
//         }
//     }