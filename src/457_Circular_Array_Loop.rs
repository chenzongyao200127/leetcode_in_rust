// 457_Circular_Array_Loop
// https://leetcode.cn/problems/circular-array-loop/


use std::collections::HashSet;
impl Solution {    
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let mut idx = vec![];
        let len = nums.len();
        for i in 0..nums.len() {
            idx.push((i as i32 + nums[i] + len as i32 * 1000)%len as i32);
        }
        let mut set: HashSet<i32> = HashSet::new();
        for i in 0..nums.len() {
            let mut direction = 1;
            if nums[i] < 0 {
                direction = -1;
            }
            set.insert(i as i32);
            let mut next = idx[i];
            loop {
                if nums[next as usize] * direction < 0 {
                    break;
                }
                if set.contains(&next) {
                    if set.len() == 1 {
                        break;
                    } else {
                        if next == idx[next as usize] { // 防止自循环
                            break;
                        } else {
                            return true;
                        }
                    }
                } else {
                    set.insert(next);
                }
                next = idx[next as usize];
            }
            set.clear();
        }

        false
    }
}


/// 示例代码
/// 注意到这张图中的每个点有且仅有一条出边，这样我们从某一个点出发，沿着单向边不断移动，最终必然会进入一个环中。
/// 而依据题目要求，我们要检查图中是否存在一个所有单向边方向一致的环。我们可以使用在无向图中找环的一个经典算法：快慢指针来解决本题
/// 具体地，我们检查每一个节点，令快慢指针从当前点出发，快指针每次移动两步，慢指针每次移动一步，
/// 期间每移动一次，我们都需要检查当前单向边的方向是否与初始方向是否一致，
/// 如果不一致，我们即可停止遍历，因为当前路径必然不满足条件。
/// 为了降低时间复杂度，我们可以标记每一个点是否访问过，过程中如果我们的下一个节点为已经访问过的节点，则可以停止遍历。
/// 在实际代码中，我们无需新建一个数组记录每个点的访问情况，而只需要将原数组的对应元素置零即可（题目保证原数组中元素不为零）。
/// 遍历过程中，如果快慢指针相遇，或者移动方向改变，那么我们就停止遍历，并将快慢指针经过的点均置零即可。
/// 
/// 
/// 
/// 这段代码实现了一个函数 `circular_array_loop`，用于判断给定的数组中是否存在循环。具体实现如下：
/// 1. 首先获取数组长度 n，并初始化一个 next 数组，其中 next[i] 表示从 i 位置出发可以到达的下一个位置。
/// 2. 对于每个 i，计算出它能够到达的下一个位置并存储在 next 数组中。
/// 3. 对于每个非零元素 nums[i]，使用快慢指针法来判断是否存在循环。如果存在，则返回 true；否则将路径上所有元素置为 0（表示已经访问过）。
/// 4. 如果没有找到任何循环，则返回 false。
/// 需要注意的是，在第二步中计算下一步时要保证结果在 [0,n) 范围内。
impl Solution {
    pub fn circular_array_loop(mut nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut next = vec![];
        for i in 0..n {
            next.push(((i as i32 + nums[i]) % n as i32 + n as i32) % n as i32); // 保证返回值在 [0,n) 中
        }
    
        for i in 0..n {
            if nums[i] == 0 {
                continue;
            }

            let mut slow = i;
            let mut fast = next[i] as usize;

            // 判断非零且方向相同
            while (nums[slow] * nums[fast]) > 0 && (nums[slow] * nums[next[fast] as usize] > 0) {
                if slow == fast {
                    if slow != next[slow] as usize {
                        return true;
                    } else {
                        break;
                    }
                }
                slow = next[slow] as usize;
                fast = next[next[fast] as usize] as usize;
            }

            // 将路径上所有元素置为 0（表示已经访问过）
            let mut add = i;
            while (nums[add] * nums[next[add] as usize]) > 0 {
                let tmp = add;
                add = next[add] as usize;
                nums[tmp] = 0;
            }
    
        }
        false
    }
}






impl Solution {
    pub fn circular_array_loop(mut nums: Vec<i32>) -> bool {
        let n=nums.len();
        let nn=n as i32;
        let next=|cur:usize,nums: &Vec<i32>|{
          ( ( (cur as i32+nums[cur] )%nn+nn)%nn) as usize
        };
        for i in 0..n{
            if nums[i]==0{
                continue
            }
            let (mut slow,mut fast)=(i,next(i,&nums));
            while nums[slow]*nums[fast]>0 && nums[slow]*nums[next(fast,&nums)]>0{ 
                if slow==fast{
                    if slow!=next(slow,&nums){
                        return true
                    }else{
                        break
                    }
                }
                slow=next(slow,&nums);
                fast=next(next(fast,&nums),&nums);
            }
            let mut add=i;
            while nums[add]*nums[next(add,&nums)]>0{
                let tmp=add;
                add=next(add,&nums);
                nums[tmp]=0;
            }
        }
        
        false
    }
}