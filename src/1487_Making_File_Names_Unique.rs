// 1487. Making File Names Unique
// https://leetcode.cn/problems/making-file-names-unique/

// 1487. 保证文件名唯一
// 给你一个长度为 n 的字符串数组 names 。你将会在文件系统中创建 n 个文件夹：在第 i 分钟，新建名为 names[i] 的文件夹。
// 由于两个文件 不能 共享相同的文件名，因此如果新建文件夹使用的文件名已经被占用，
// 系统会以 (k) 的形式为新文件夹的文件名添加后缀，其中 k 是能保证文件名唯一的 最小正整数 。
// 返回长度为 n 的字符串数组，其中 ans[i] 是创建第 i 个文件夹时系统分配给该文件夹的实际名称。

// Failed...
// use std::collections::HashMap;
// impl Solution {
//     pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
//         let mut map: HashMap<String, usize> = HashMap::new();
//         let mut ans: Vec<String> = vec![];
//         // let mut new_name = "".to_string();
//         let mut tmp = String::new();
//         for file in names.iter() {
//             if let Some(val) = map.get(&file[..]) {
//                 let mut new_name = file.clone();
//                 new_name.push('(');
//                 new_name.push_str(&(val.to_string()));
//                 new_name.push(')');
//                 ans.push(new_name.clone());
//                 // println!("{:?}", file);
//                 map.insert(file.to_string(), val + 1);
//                 tmp = new_name.clone();
//                 map.insert(tmp ,1);
//                 // let tmp = ans[ans.len()-1].clone();
//                 // map.insert(&tmp, 1);
//             } else {
//                 map.insert(file.to_string(), 1);
//                 ans.push(file.to_string());
//             }
//             if file.ends_with(')') {
//                 let pre_name = get_file_name(&ans[ans.len()-1]);
//                 // println!("pre_name:{:?}", pre_name);
//                 map.entry(pre_name.to_string()).and_modify(|cnt| *cnt += 1);
//             }
//             // println!("{:?}", map);
//         }

//         ans
//     }
// }

// pub fn get_file_name(name: &str) -> &str {
//     let n_chars: Vec<char> = name.chars().collect();
//     for i in (0..name.len()).rev() {
//         if n_chars[i] == '(' {
//             return &name[0..i];
//         }
//     }
//     name
// }

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut tab = HashMap::with_capacity(names.len());
        names
            .into_iter()
            .map(|s| {
                if tab.get(&s).is_some() {
                    let mut k = *tab.get(&s).unwrap();
                    let mut ans = format!("{s}({k})");
                    while tab.get(&ans).is_some() {
                        k += 1;
                        ans = format!("{s}({k})");
                    }
                    tab.insert(ans.clone(), 1);
                    *tab.get_mut(&s).unwrap() = k + 1;
                    ans
                } else {
                    tab.insert(s.clone(), 1);
                    s
                }
            })
            .collect()
    }
}


impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut d = std::collections::HashMap::new();
        names.into_iter().map(|name| {
            let mut s = name.clone();
            while d.contains_key(&s) {
                s = format!("{}({})", name, d[&name]);
                *d.get_mut(&name).unwrap() += 1;
            }
            d.insert(s.clone(), 1);
            s
        }).collect()
    }
}
