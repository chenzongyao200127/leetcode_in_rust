# 2477_到达首都的最少油耗
# https://leetcode.cn/problems/minimum-fuel-cost-to-report-to-the-capital/description/

from typing import List

class Solution:
    def minimumFuelCost(self, roads: List[List[int]], seats: int) -> int:
        if not roads:
            return 0

        def buildGraph(roads: List[List[int]]) -> dict:
            graph = {}
            for road in roads:
                graph.setdefault(road[0], []).append(road[1])
                graph.setdefault(road[1], []).append(road[0])
            return graph
                
        graph = buildGraph(roads)
        visited = set()
        visited.add(0)       
        cost = 0             
            
        def dfs(node):
            # add node to visited
            visited.add(node)
            neigs = graph.get(node, [])
            filterNeigs = [x for x in neigs if x not in visited]
            
            nonlocal cost
                        
            # edge
            if not filterNeigs:
                cost += 1
                return 1
            
            people = 0
            
            for n in filterNeigs:
                people += dfs(n)
            
            if (people + 1) % seats == 0:
                cost += (people + 1) // seats
            else:
                cost += 1 + (people + 1) // seats
            
            return people + 1
                
        neigs = graph.get(0, [])
        # print(neigs)
        for root in neigs:
            dfs(root)
        
        return cost
            
    from typing import List



s = Solution()
ans = s.minimumFuelCost(roads = [[3,1],[3,2],[1,0],[0,4],[0,5],[4,6]], seats = 2)
print(ans)            


s = Solution()
ans = s.minimumFuelCost(roads = [[0,1],[0,2],[0,3]], seats = 5)
print(ans)    


s = Solution()
ans = s.minimumFuelCost(roads = [[0,1],[2,0]], seats = 1)
print(ans)    


s = Solution()
ans = s.minimumFuelCost(roads = [[1,0],[0,2],[3,1],[1,4],[5,0]], seats = 1)
print(ans)    


class Solution:
    def minimumFuelCost(self, roads: List[List[int]], seats: int) -> int:
        g = [[] for _ in range(len(roads) + 1)]
        for x, y in roads:
            g[x].append(y)  # 记录每个点的邻居
            g[y].append(x)

        ans = 0
        def dfs(x: int, fa: int) -> int:
            size = 1
            for y in g[x]:
                if y != fa:  # 递归子节点，不能递归父节点
                    size += dfs(y, x)  # 统计子树大小
            if x:  # x 不是根节点
                nonlocal ans
                ans += (size - 1) // seats + 1  # ceil(size/seats)
            return size
        dfs(0, -1)
        return ans
    
# rust  
# impl Solution {
#     pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
#         let mut g = vec![vec![]; roads.len() + 1];
#         for e in &roads {
#             let x = e[0] as usize;
#             let y = e[1] as usize;
#             g[x].push(y); // 记录每个点的邻居
#             g[y].push(x);
#         }
#         let mut ans = 0i64;
#         Self::dfs(0, 0, &g, seats, &mut ans);
#         ans
#     }

#     fn dfs(x: usize, fa: usize, g: &Vec<Vec<usize>>, seats: i32, ans: &mut i64) -> i32 {
#         let mut size = 1;
#         for &y in &g[x] {
#             if y != fa { // 递归子节点，不能递归父节点
#                 size += Self::dfs(y, x, g, seats, ans); // 统计子树大小
#             }
#         }
#         if x != 0 { // x 不是根节点
#             *ans += ((size - 1) / seats + 1) as i64; // ceil(size/seats)
#         }
#         size
#     }
# }

# 作者：灵茶山艾府
# 链接：https://leetcode.cn/problems/minimum-fuel-cost-to-report-to-the-capital/description/
# 来源：力扣（LeetCode）
# 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。    