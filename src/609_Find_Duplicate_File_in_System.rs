// 609. Find Duplicate File in System
// https://leetcode.cn/problems/find-duplicate-file-in-system/


// 609. 在系统中查找重复文件
// 给你一个目录信息列表 paths ，包括目录路径，以及该目录中的所有文件及其内容，请你按路径返回文件系统中的所有重复文件。答案可按 任意顺序 返回。
// 输入 列表中的单个目录信息字符串的格式如下：
// "root/d1/d2/.../dm f1.txt(f1_content) f2.txt(f2_content) ... fn.txt(fn_content)"
// 这意味着，在目录 root/d1/d2/.../dm 下，有 n 个文件 ( f1.txt, f2.txt ... fn.txt ) 的内容分别是 
// ( f1_content, f2_content ... fn_content ) 。注意：n >= 1 且 m >= 0 。如果 m = 0 ，则表示该目录是根目录。
// 输出 是由 重复文件路径组 构成的列表。其中每个组由所有具有相同内容文件的文件路径组成。文件路径是具有下列格式的字符串：
// "directory_path/file_name.txt"

use std::collections::HashMap;
use std::iter::FromIterator;

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut dict = HashMap::<String, Vec<String>>::new();
        for path in paths {
            let path: Vec<&str> = path.split(' ').collect();
            let dir = path[0];
            for i in 1..path.len() {
                let op = path[i].find('(').unwrap();
                let cp = path[i].find(')').unwrap();
                let content = path[i][op + 1..cp].to_string();
                let mut full_path = dir.to_string();
                full_path.push('/');
                full_path.push_str(&path[i][..op]);
                dict.entry(content).or_default().push(full_path);
            }
        }
        Vec::from_iter(dict.values().filter(|v| v.len() >= 2).cloned())
    }
}