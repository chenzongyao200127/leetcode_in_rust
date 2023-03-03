// 228. Summary Ranges
// https://leetcode.cn/problems/summary-ranges/

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return vec![];
        }

        let mut ans = vec![];
        let mut pre = nums[0];
        let mut start = nums[0];
        for i in 1..nums.len() {
            if nums[i] == pre + 1 {
                pre = nums[i];
                continue;
            } else {
                if pre == start {
                    ans.push(start.to_string());
                } else {
                    let mut s = start.to_string();
                    s.push_str(&("->".to_string()));
                    s.push_str(&(pre.to_string()));
                    ans.push(s);
                }
                start = nums[i];
                pre = nums[i];
            }
        }
        if pre == start {
            ans.push(start.to_string());
        } else {
            let mut s = start.to_string();
            s.push_str(&("->".to_string()));
            s.push_str(&(pre.to_string()));
            ans.push(s);
        }
    
        ans
    }
}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
 if nums.len()==0{
        return Vec::new()
    }
    let mut list1:Vec<&[i32]>=Vec::new();
    let mut list2:Vec<String>=Vec::new();
    let mut start=0;
    for i in 0..nums.len(){
        let mut slice=&nums[..];
        if i!=0&&nums[i]>nums[i-1]+1{
            slice=&nums[start..i];
            list1.push(slice);
            start=i;
        }
    }
    list1.push(&nums[start..nums.len()]);
    for slice in list1.iter(){
        if slice.len()==1{
            list2.push(slice[0].to_string())
        }else{
            list2.push(slice[0].to_string()+"->"+&slice[slice.len()-1].to_string());
        }
    }
    //println!("{:?}",list2);
    list2
    }
}