// 220. Contains Duplicate III
// https://leetcode.cn/problems/contains-duplicate-iii/

// 我们思考这样一个问题:
// 某天老师让全班同学各自说出自己的出生日期，然后统计一下出生日期相差小于等于30天的同学。我们很容易想到，出生在同一个月的同学，一定满足上面的条件。出生在相邻月的同学，也有可能满足那个条件，这就需要计算一下来确定了。但如果月份之间相差了两个月，那就不可能满足这个条件了。
// 例如某同学出生于6月10日，其他6月出生的同学，都与其相差小于30天。另一些5月20日和7月1日的同学也满足条件。但是4月份的和8月份的同学就不可能满足条件了。

// https://leetcode.cn/problems/contains-duplicate-iii/solution/c-li-yong-tong-fen-zu-xiang-xi-jie-shi-b-ofj6/

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        // NOTE 桶排序的好题，通过将数字划分为若干个区间，来判断是否滑动窗口内是否存在value_diff范围内的数字对
        // 桶的大小为value_diff，作为整除的除数因此加一
        let value_diff = value_diff as i64 + 1;
        let mut num_bucket = std::collections::HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let key = Self::get_id(num, value_diff);
            if num_bucket.contains_key(&key) {
                return true;
            }
            num_bucket.insert(key, num);

            // 检查相邻的区间有没有符合要求的数字
            if matches!(num_bucket.get(&(key-1)),Some(&prev_num)if (prev_num-num).abs()<value_diff as i32)
            {
                return true;
            }
            if matches!(num_bucket.get(&(key+1)),Some(&prev_num)if (prev_num-num).abs()<value_diff as i32)
            {
                return true;
            }

            if i >= index_diff as usize {
                num_bucket.remove(&Self::get_id(nums[i - index_diff as usize], value_diff));
            }
        }
        false
    }
    // NOTE 通过整除将数字划分到不同的桶中，为了避免溢出，这里value_diff是i64
    fn get_id(num: i32, value_diff: i64) -> i32 {
        let ret = if num >= 0 {
            num as i64 / value_diff
        } else {
            (num + 1) as i64 / value_diff - 1
        };
        ret as i32
    }
}


// class Solution {
//     public:
//         long getID(long x, long t){
//             //如果x元素大于等于零,直接分桶
//             if(x>=0){
//                 return x / ( t + 1 );
//             }else{
//             //如果x元素小于零,偏移后再分桶
//                 return ( x + 1 )/( t + 1 ) - 1 ;
//             }
//             return 0;
//         }
//         bool containsNearbyAlmostDuplicate(vector<int>& nums, int k, int t) {
//             int n = nums.size();
//             //我们用unordered_map来实现桶,其底层实现是一个哈希表.
//             unordered_map<int,int> m;
//             for(int i = 0 ; i < n; i++ ){
//                 //当前元素
//                 long  x = nums[i];
//                 //给当前元素生成id,这里我们同一个id的桶内元素满足abs(nums[i] - nums[j]) <= t
//                 int id = getID(x,t);
//                 //前面的i-(k+1)是超出了范围的桶,我们把它提前删除,以免影响判断
//                 if( i-(k+1) >= 0 ){
//                     //把下标不满足我们判断范围的桶删除了
//                     m.erase(getID(nums[i-(k+1)],t));
//                 }
//                 //看看当前元素属于的桶中有没有元素
//                 if(m.find(id)!=m.end()){
//                     return true;
//                 }
//                 //看看前面相邻桶有没有符合条件的
//                 if(m.find(id - 1) != m.end() && abs(m[id-1]-x) <= t){
//                     return true;
//                 }
//                 //看看后面相邻桶有没有符合条件的
//                 if(m.find(id + 1) != m.end() && abs(m[id+1]-x) <= t){
//                     return true;
//                 }
//                 //分桶,把这个元素放入其属于的桶
//                 m[id] = x;
//             }
//             return false;
//         }
//     };
    
    // 作者：limaodaxia
    // 链接：https://leetcode.cn/problems/contains-duplicate-iii/solution/c-li-yong-tong-fen-zu-xiang-xi-jie-shi-b-ofj6/
    // 来源：力扣（LeetCode）
    // 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。


// 超出时间限制
use std::collections::HashSet;
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let mut s: HashSet<i32> = HashSet::new();
        for (i, val) in nums.iter().enumerate() {
            for diff_y in 0..=value_diff {
                if s.contains(&(val+diff_y)) || s.contains(&(val-diff_y)) { // to be optimised
                    return true;
                }
            }
    
            s.insert(*val);
            if s.len() >= (index_diff + 1) as usize {
                let offset = i - index_diff as usize;
                s.remove(&nums[offset]);
            }
        }
        false
    }
}

/// 方法一：滑动窗口 + 有序集合
/// 如果使用队列维护滑动窗口内的元素，由于元素是无序的，我们只能对于每个元素都遍历一次队列来检查是否有元素符合条件。
/// 如果数组的长度为n，则使用队列的时间复杂度为 O(nk)会超出时间限制。
/// 因此我们希望能够找到一个数据结构维护滑动窗口内的元素，该数据结构需要满足以下操作：
/// 支持添加和删除指定元素的操作，否则我们无法维护滑动窗口；
/// 内部元素有序，支持二分查找的操作，这样我们可以快速判断滑动窗口中是否存在元素满足条件
/// 我们可以使用有序集合来支持这些操作。
// class Solution {
//     public:
//         bool containsNearbyAlmostDuplicate(vector<int>& nums, int k, int t) {
//             int n = nums.size();
//             set<int> rec;
//             for (int i = 0; i < n; i++) {
//                 auto iter = rec.lower_bound(max(nums[i], INT_MIN + t) - t); //lower_bound() 函数用于在指定区域内查找不小于目标值的第一个元素。也就是说，使用该函数在指定范围内查找某个目标值时，最终查找到的不一定是和目标值相等的元素，还可能是比目标值大的元素。
//                 if (iter != rec.end() && *iter <= min(nums[i], INT_MAX - t) + t) {
//                     return true;
//                 }
//                 rec.insert(nums[i]);
//                 if (i >= k) {
//                     rec.erase(nums[i - k]);
//                 }
//             }
//             return false;
//         }
//     };

// class Solution {
//     public boolean containsNearbyAlmostDuplicate(int[] nums, int k, int t) {
//         int n = nums.length;
//         TreeSet<Long> set = new TreeSet<Long>();
//         for (int i = 0; i < n; i++) {
//             Long ceiling = set.ceiling((long) nums[i] - (long) t);
//             if (ceiling != null && ceiling <= (long) nums[i] + (long) t) {
//                 return true;
//             }
//             set.add((long) nums[i]);
//             if (i >= k) {
//                 set.remove((long) nums[i - k]);
//             }
//         }
//         return false;
//     }
// }


impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let mut nums: Vec<_> = nums
            .iter()
            .enumerate()
            .map(|(idx, &x)| (x as i64, idx as i32))
            .collect();

        nums.sort_unstable();

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[j].0 - nums[i].0 > t as i64 {
                    break;
                }
                if (nums[i].1 - nums[j].1).abs() <= k {
                    return true;
                }
            }
        }

        false
    }
}

// use std::collections::HashSet;
// impl Solution {
//     pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
//         let mut s: HashSet<i32> = HashSet::new();
//         for (i, val) in nums.iter().enumerate() {
//             if s.contains(val) {
//                 return true;
//             }

//             s.insert(*val);
//             if s.len() >= (k + 1) as usize {
//                 let offset = i - k as usize;
//                 s.remove(&nums[offset]);
//             }
//         }
//         false
//     }
// }