# 360th_contest

# 给你两个正整数 low 和 high 。
# 对于一个由 2 * n 位数字组成的整数 x ，如果其前 n 位数字之和与后 n 位数字之和相等，则认为这个数字是一个对称整数。
# 返回在 [low, high] 范围内的 对称整数的数目 。

# 示例 1：
# 输入：low = 1, high = 100
# 输出：9
# 解释：在 1 到 100 范围内共有 9 个对称整数：11、22、33、44、55、66、77、88 和 99 。

# 示例 2：
# 输入：low = 1200, high = 1230
# 输出：4
# 解释：在 1200 到 1230 范围内共有 4 个对称整数：1203、1212、1221 和 1230 。

class Solution:
    # 定义一个方法来计算在给定范围内的对称整数的数量
    def countSymmetricIntegers(self, low: int, high: int) -> int:

        # 定义一个内部函数来计算数字的各位之和
        def get_digit_sum(num: int) -> int:
            # 使用列表推导式将数字的每一位转换成整数，并求和返回
            return sum(int(digit) for digit in str(num))

        # 定义一个内部函数判断一个数字是否是“对称”的
        # 在这里，一个对称的数字是指其前半部分的数字和等于后半部分的数字和
        def is_symmetric(num: int) -> bool:
            s = str(num)
            # 计算数字的长度的一半，这将用于分割数字
            half_length = len(s) // 2
            # 检查数字的前半部分和后半部分的各位之和是否相等
            return get_digit_sum(s[:half_length]) == get_digit_sum(s[half_length:])

        # 对给定范围内的所有数字进行迭代
        # 如果数字长度是偶数，并且数字是对称的，则进行累加
        # 使用列表推导式并返回累加的结果
        return sum(is_symmetric(num) for num in range(low, high + 1) if len(str(num)) % 2 == 0)

# Rust
# impl Solution {
#     pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
#         // Function to compute the sum of the digits of a number
#         fn get_digit_sum(num: i32) -> i32 {
#             num.to_string().chars().map(|ch| ch.to_digit(10).unwrap() as i32).sum()
#         }

#         // Function to determine if a number is symmetric, based on its digits
#         fn is_symmetric(num: i32) -> bool {
#             let s = num.to_string();
#             let half_length = s.len() / 2;
#             get_digit_sum(s[0..half_length].parse().unwrap()) == get_digit_sum(s[half_length..].parse().unwrap())
#         }

#         // Iterate over the range of numbers, filter the even-length numbers, 
#         // check if they are symmetric, and then count the number of such symmetric numbers
#         (low..=high)
#             .filter(|&num| num.to_string().len() % 2 == 0)
#             .filter(|&num| is_symmetric(num))
#             .count() as i32
#     }
# }
    
    
# 给你一个下标从 0 开始的字符串 num ，表示一个非负整数。
# 在一次操作中，您可以选择 num 的任意一位数字并将其删除。请注意，如果你删除 num 中的所有数字，则 num 变为 0。
# 返回最少需要多少次操作可以使 num 变成特殊数字。
# 如果整数 x 能被 25 整除，则该整数 x 被认为是特殊数字。

 
# 示例 1：
# 输入：num = "2245047"
# 输出：2
# 解释：删除数字 num[5] 和 num[6] ，得到数字 "22450" ，可以被 25 整除。
# 可以证明要使数字变成特殊数字，最少需要删除 2 位数字。

# 示例 2：
# 输入：num = "2908305"
# 输出：3
# 解释：删除 num[3]、num[4] 和 num[6] ，得到数字 "2900" ，可以被 25 整除。
# 可以证明要使数字变成特殊数字，最少需要删除 3 位数字。

# 示例 3：
# 输入：num = "10"
# 输出：1
# 解释：删除 num[0] ，得到数字 "0" ，可以被 25 整除。
# 可以证明要使数字变成特殊数字，最少需要删除 1 位数字。
    
    
class Solution:
    # 定义一个方法计算为了使字符串`num`末尾变为某个特定的倍数，最少需要删除多少字符
    def minimumOperations(self, num: str) -> int:
        
        # 定义一个内部函数来找到目标字符target1和target2的最后一个出现位置
        # target1必须出现在target2之前
        def find_last_two_indices(target1, target2):
            # 查找target2的最后一个出现的位置
            idx2 = num.rfind(target2)
            # 如果没找到，返回-1, -1
            if idx2 == -1:
                return -1, -1
            # 在target2之前的字符串中查找target1的最后一个出现位置
            idx1 = num.rfind(target1, 0, idx2)
            return idx1, idx2

        # 定义一个函数计算需要删除的字符数，以便在idx1和idx2之间没有其他字符
        def count_deletions(idx1, idx2):
            # 如果idx1或idx2为-1，返回正无穷（表示不可能）
            if idx1 == -1 or idx2 == -1:
                return float('inf')
            # 计算需要删除的字符数
            return len(num) - 1 - idx2 + idx2 - idx1 - 1

        # 如果字符串长度为1，则判断是否为'0'
        if len(num) == 1:
            return 1 if num != "0" else 0

        # 如果num中包含'0'，则单独考虑删除到只剩下一个'0'的情况
        single_zero = len(num) - 1 if '0' in num else float('inf')

        # 寻找使num末尾为00、25、50、75的情况
        idx00 = find_last_two_indices('0', '0')
        idx25 = find_last_two_indices('2', '5')
        idx50 = find_last_two_indices('5', '0')
        idx75 = find_last_two_indices('7', '5')

        # 返回所有可能的情况中，所需删除的最少字符数
        return min(single_zero, count_deletions(*idx00), count_deletions(*idx25), count_deletions(*idx50), count_deletions(*idx75), len(num))

# Rust
# impl Solution {
#     pub fn minimum_operations(num: String) -> i32 {
#         // Function to find the last occurrences of two target characters
#         fn find_last_two_indices(num: &str, target1: char, target2: char) -> (i32, i32) {
#             let idx2 = num.rfind(target2).unwrap_or(usize::MAX);
#             if idx2 == usize::MAX {
#                 return (-1, -1);
#             }
#             let idx1 = num[..idx2 as usize].rfind(target1).unwrap_or(usize::MAX);
#             (idx1 as i32, idx2 as i32)
#         }

#         // Function to count deletions required to ensure no other characters between idx1 and idx2
#         fn count_deletions(num_len: usize, idx1: i32, idx2: i32) -> i32 {
#             if idx1 == -1 || idx2 == -1 {
#                 return std::i32::MAX;
#             }
#             ((num_len - 1) as i32 - idx2 + idx2 - idx1 - 1) as i32
#         }

#         let num_len = num.len();
        
#         // For a single character string
#         if num_len == 1 {
#             return if num != "0" { 1 } else { 0 };
#         }

#         // Special case for a single '0' in the string
#         let single_zero = if num.contains('0') {
#             (num_len - 1) as i32
#         } else {
#             std::i32::MAX
#         };

#         // Find the indices for the combinations '00', '25', '50', and '75'
#         let idx00 = find_last_two_indices(&num, '0', '0');
#         let idx25 = find_last_two_indices(&num, '2', '5');
#         let idx50 = find_last_two_indices(&num, '5', '0');
#         let idx75 = find_last_two_indices(&num, '7', '5');

#         // Return the minimum of all the scenarios
#         [
#             single_zero,
#             count_deletions(num_len, idx00.0, idx00.1),
#             count_deletions(num_len, idx25.0, idx25.1),
#             count_deletions(num_len, idx50.0, idx50.1),
#             count_deletions(num_len, idx75.0, idx75.1),
#             num_len as i32,
#         ]
#         .iter()
#         .cloned()
#         .min()
#         .unwrap()
#     }
# }


# 给你一个下标从 0 开始的整数数组 nums ，以及整数 modulo 和整数 k 。
# 请你找出并统计数组中 趣味子数组 的数目。
# 如果 子数组 nums[l..r] 满足下述条件，则称其为 趣味子数组 ：
# 在范围 [l, r] 内，设 cnt 为满足 nums[i] % modulo == k 的索引 i 的数量。并且 cnt % modulo == k 。
# 以整数形式表示并返回趣味子数组的数目。
# 注意：子数组是数组中的一个连续非空的元素序列。

 
# 示例 1：
# 输入：nums = [3,2,4], modulo = 2, k = 1
# 输出：3
# 解释：在这个示例中，趣味子数组分别是： 
# 子数组 nums[0..0] ，也就是 [3] 。 
# - 在范围 [0, 0] 内，只存在 1 个下标 i = 0 满足 nums[i] % modulo == k 。
# - 因此 cnt = 1 ，且 cnt % modulo == k 。
# 子数组 nums[0..1] ，也就是 [3,2] 。
# - 在范围 [0, 1] 内，只存在 1 个下标 i = 0 满足 nums[i] % modulo == k 。
# - 因此 cnt = 1 ，且 cnt % modulo == k 。
# 子数组 nums[0..2] ，也就是 [3,2,4] 。
# - 在范围 [0, 2] 内，只存在 1 个下标 i = 0 满足 nums[i] % modulo == k 。
# - 因此 cnt = 1 ，且 cnt % modulo == k 。
# 可以证明不存在其他趣味子数组。因此，答案为 3 。

# 示例 2：
# 输入：nums = [3,1,9,6], modulo = 3, k = 0
# 输出：2
# 解释：在这个示例中，趣味子数组分别是： 
# 子数组 nums[0..3] ，也就是 [3,1,9,6] 。
# - 在范围 [0, 3] 内，只存在 3 个下标 i = 0, 2, 3 满足 nums[i] % modulo == k 。
# - 因此 cnt = 3 ，且 cnt % modulo == k 。
# 子数组 nums[1..1] ，也就是 [1] 。
# - 在范围 [1, 1] 内，不存在下标满足 nums[i] % modulo == k 。
# - 因此 cnt = 0 ，且 cnt % modulo == k 。
# 可以证明不存在其他趣味子数组，因此答案为 2 。

from typing import List

class Solution:
    # 定义方法来计算符合条件的子数组的数量
    def countInterestingSubarrays(self, nums: List[int], modulo: int, k: int) -> int:
        
        # 初始化一个列表，存储到当前索引的前缀和，长度比nums大1，方便计算
        prefix = [0] * (len(nums) + 1)
        
        # 初始化一个字典，键是前缀和取模后的结果，值是这种结果出现的次数。初始化{0: 1}是为了处理第一个元素的情况。
        count = {0: 1}
        
        # 结果变量，存储满足条件的子数组数量
        res = 0

        # 遍历nums列表
        for i in range(len(nums)):
            # 如果当前数取模得到的结果是k，则在前缀和上加1
            if nums[i] % modulo == k:
                prefix[i + 1] = prefix[i] + 1
            # 否则，维持前缀和不变
            else:
                prefix[i + 1] = prefix[i]
            
            # 计算目标值，即我们期望在之前的前缀和中出现多少次
            target = (prefix[i + 1] - k) % modulo
            
            # 如果目标值在之前出现过，那么将出现的次数加到结果上
            res += count.get(target, 0)
            
            # 更新当前前缀和的计数
            # 也就是说，这个前缀和的模结果出现过了，所以我们在字典中加1
            count[prefix[i + 1] % modulo] = count.get(prefix[i + 1] % modulo, 0) + 1

        # 返回满足条件的子数组的数量
        return res  


# LCA 【模板】最近公共祖先
from typing import List

class TreeAncestor:
    def __init__(self, edges: List[List[int]]):
        # 树中节点的总数
        n = len(edges) + 1
        
        # 树的最大高度。m是我们可以使用二进制提升上跳的层数。
        m = n.bit_length()
        
        # 构造邻接列表来表示树
        g = [[] for _ in range(n)]
        for x, y in edges:
            g[x].append(y)
            g[y].append(x)

        # 深度数组，存储每个节点的深度/层级
        depth = [0] * n
        
        # 表格存储每个节点在二的幂次上的祖先。即，pa[u][i]给出了节点u的2^i-th祖先。
        pa = [[-1] * m for _ in range(n)]
        
        # 深度优先搜索来为每个节点填充深度和直接的父节点
        def dfs(x: int, parent: int) -> None:
            pa[x][0] = parent
            for y in g[x]:
                if y != parent:
                    depth[y] = depth[x] + 1
                    dfs(y, x)
        
        dfs(0, -1)

        # 预处理阶段：使用二进制提升方法填充祖先表
        for i in range(1, m):
            for x in range(n):
                if pa[x][i - 1] != -1:
                    # 关键步骤
                    pa[x][i] = pa[pa[x][i - 1]][i - 1]

        # 为以后的使用存储深度和父数组
        self.depth = depth
        self.pa = pa

    def get_kth_ancestor(self, node: int, k: int) -> int:
        # 遍历k的每一位。根据位值沿树上移。
        for i in range(k.bit_length()):
            if (k >> i) & 1:
                node = self.pa[node][i]
                if node == -1:  # 检查以避免访问无效索引
                    break
        return node

    def get_lca(self, x: int, y: int) -> int:
        # 如果节点x比节点y更深，交换它们
        if self.depth[x] > self.depth[y]:
            x, y = y, x

        # 将y带到与x相同的深度
        y = self.get_kth_ancestor(y, self.depth[y] - self.depth[x])
        
        # 如果x和y相同，则x（或y）是LCA
        if y == x:
            return x
        
        # 从最高的祖先开始，我们向下移动树，直到找到最直接的共同祖先。
        for i in range(len(self.pa[x]) - 1, -1, -1):
            if self.pa[x][i] != self.pa[y][i]:
                x, y = self.pa[x][i], self.pa[y][i]
        
        # 返回x（或y）的直接祖先作为最低共同祖先。
        return self.pa[x][0]




# from typing import List

# class Solution:
#     def minOperationsQueries(self, n: int, edges: List[List[int]], queries: List[List[int]]) -> List[int]:
#         # 构造邻接表
#         g = [[] for _ in range(n)]
#         for x, y, w in edges:
#             g[x].append((y, w - 1))
#             g[y].append((x, w - 1))

#         m = n.bit_length()
#         pa = [[-1] * m for _ in range(n)]
#         # cnt 用于存储每个节点到其2^i-th祖先路径中每种权重的计数
#         cnt = [[[0] * 26 for _ in range(m)] for _ in range(n)]
#         depth = [0] * n

#         # DFS 初始化每个节点的直接父节点和该边的权重计数
#         def dfs(x: int, fa: int) -> None:
#             pa[x][0] = fa
#             for y, w in g[x]:
#                 if y != fa:
#                     cnt[y][0][w] = 1
#                     depth[y] = depth[x] + 1
#                     dfs(y, x)
#         dfs(0, -1)

#         # 使用二进制提升预处理每个节点的2^i-th祖先和该路径上的权重计数
#         for i in range(1, m):
#             for x in range(n):
#                 p = pa[x][i - 1]
#                 if p != -1:
#                     pa[x][i] = pa[p][i - 1]
#                     for j in range(26):
#                         cnt[x][i][j] = cnt[x][i - 1][j] + cnt[p][i - 1][j]

#         ans = []
#         for x, y in queries:
#             # 初始化路径长度和权重
#             path_len = depth[x] + depth[y]
#             cw = [0] * 26

#             # 如果x更深，交换x和y
#             if depth[x] > depth[y]:
#                 x, y = y, x

#             # 调整y的深度使其与x相同，同时更新权重
#             k = depth[y] - depth[x]
#             for i in range(k.bit_length()):
#                 if (k >> i) & 1:
#                     for j in range(26):
#                         cw[j] += cnt[y][i][j]
#                     y = pa[y][i]

#             # 当x和y不相同时，同步地向上移动它们直到它们相遇
#             if y != x:
#                 for i in range(m - 1, -1, -1):
#                     if pa[x][i] != pa[y][i]:
#                         for j in range(26):
#                             cw[j] += cnt[x][i][j] + cnt[y][i][j]
#                         x, y = pa[x][i], pa[y][i]

#                 # 添加x和y的直接父节点的颜色计数
#                 for j in range(26):
#                     cw[j] += cnt[x][0][j] + cnt[y][0][j]

#                 x = pa[x][0]

#             # 减去lca深度的两倍以得到真正的路径长度
#             lca = x
#             path_len -= 2 * depth[lca]

#             # 添加结果
#             ans.append(path_len - max(cw))

#         return ans

        
# s = Solution()
# ans = s.minOperationsQueries(n = 7, edges = [[0,1,1],[1,2,1],[2,3,1],[3,4,2],[4,5,2],[5,6,2]], queries = [[0,3],[3,6],[2,6],[0,6]])
# print(ans) # [0,0,1,3]

# s = Solution()
# ans = s.minOperationsQueries(n = 8, edges = [[1,2,6],[1,3,4],[2,4,6],[2,5,3],[3,6,6],[3,0,8],[7,0,2]], queries = [[4,6],[0,4],[6,5],[7,4]])
# print(ans) # [1,2,2,3]

# s = Solution()
# ans = s.minOperationsQueries(n = 1, edges = [], queries = [[0,0]])
# print(ans) 
