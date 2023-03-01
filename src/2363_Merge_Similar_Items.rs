// 2363. Merge Similar Items
// https://leetcode.cn/problems/merge-similar-items/

use std::collections::HashMap;
impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for vec in items1 {
            map.insert(vec[0], vec[1]);
        }
        for vec in items2 {
            map.entry(vec[0]).and_modify(|weight| *weight += vec[1]).or_insert(vec[1]);
        }
        let mut ans = vec![];
        for (key, value) in map {
            let mut tmp = vec![];
            tmp.push(key);
            tmp.push(value);
            ans.push(tmp);
        }
    
        ans.sort_by_key(|x| x[0].abs());
        ans
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut items = items1.iter().map(|item| (item[0], item[1])).collect::<HashMap<_, _>>();
        items2.iter().for_each(|item| *items.entry(item[0]).or_insert(0) += item[1]);
        let mut item_list = items.iter().map(|(&value, &weight)| vec![value, weight]).collect::<Vec<_>>();
        item_list.sort_by_key(|item| item[0]);
        item_list
    }
}