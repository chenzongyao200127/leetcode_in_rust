// 466. Count The Repetitions
// https://leetcode.cn/problems/count-the-repetitions/


// 法2：暴力解做优化
impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let str1: Vec<char> = s1.chars().collect();
        let str2: Vec<char> = s2.chars().collect();
        let mut dp = vec![0; 1000001];
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
}

// 法3：加速法2中循环节的查找
/// 由于法2中循环节的含义是：n个s1正好能够完全匹配m个s2，
/// 能要查找很多个s1才能找到满足条件的循环节。
/// 因此，要想办法加速整个过程，重新定义循环节的含义，
/// 使得匹配更少的s1又能找到便于计算最终答案的循环节。
/// 这就是官方题解的做法了，我们可以将不断循环的 s2 组成字符串S2，
/// 然后去找是否存在一个子串，即「循环节」，满足不断在 S2 中循环，且这个循环节能对应固定数量的 s1。
// class Solution {
//     public:
//         int getMaxRepetitions(string s1, int n1, string s2, int n2) {
//             /**
//               * 以 s2 字符串的下标 index 为索引，
//               * 存储匹配至 s1 的末尾时，已经匹配过的s1 的个数 s1cnt 和 s2 的个数 s2cnt 
//               * 表示匹配到 s1cnt 个 s1 末尾时，已经匹配到了 s2cnt 个 s2 中的第 index 个字符
//               **/
//             unordered_map<int,pair<int,int>> dp;
//             /**
//               * pre_loop进入循环节前匹配了多少个s1和s2
//               * in_loop一个循环节能匹配多少个s1和s2
//               **/
//             pair<int,int> pre_loop,in_loop;
//             int idx = 0;
//             int s1cnt=0,s2cnt=0;
//             while(true){
//                 ++s1cnt;
//                 for(int i=0;i<s1.length();i++){
//                     if(s1[i]==s2[idx]){
//                         idx++;
//                         if(idx==s2.length()){
//                             idx=0;
//                             s2cnt++;
//                         }
//                     }
//                 }
//                 if(s1cnt==n1){
//                     return s2cnt/n2;
//                 }
//                 if(dp.count(idx)){
//                     pre_loop=dp[idx];
//                     in_loop=make_pair(s1cnt-pre_loop.first,s2cnt-pre_loop.second);
//                     break;
//                 }
//                 else dp[idx]=make_pair(s1cnt,s2cnt);
//             }
//             int ans=pre_loop.second+(n1-pre_loop.first)/in_loop.first*in_loop.second;
//             for(int loop=(n1-pre_loop.first)%in_loop.first;loop>0;loop--){
//                 for(int i=0;i<s1.length();i++){
//                     if(s1[i]==s2[idx]){
//                         idx++;
//                         if(idx==s2.length()){
//                             idx=0;
//                             ans++;
//                         }
//                     }
//                 }
//             }
//             return ans/n2;
//         }
//     };

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {

    }
}


/// 法4：另起炉灶，匹配总字符
/// 最后来一个不一样的思路，前3种方案都是查找循环节，该方法尝试匹配总字符。
/// 假设dp数组中dp[i]表示从s2的i字符开始匹配s1，可以匹配多少个字符。
/// 然后用dp数组做转移，统计从第1个s1开始，一直到第n1个s1，可以一共匹配s2中的多少个字符，再除以s2长度和n2即可。
