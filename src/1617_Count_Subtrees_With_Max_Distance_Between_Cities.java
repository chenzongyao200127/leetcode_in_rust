// 1617_Count_Subtrees_With_Max_Distance_Between_Cities

// 输入：n = 4, edges = [[1,2],[2,3],[2,4]]
// 输出：[3,4,0]
// 解释：
// 子树 {1,2}, {2,3} 和 {2,4} 最大距离都是 1 。
// 子树 {1,2,3}, {1,2,4}, {2,3,4} 和 {1,2,3,4} 最大距离都为 2 。
// 不存在城市间最大距离为 3 的子树。

import java.util.*;     //this will include all classes from java.util package

// 1. DP
// 检测树的连通性 (深度优先搜索或者广度优先搜索来检测连通性即可)
// 计算树的直径 (树形动态规划)
class Solution {
    int mask;
    int diameter;

    public int[] countSubgraphsForEachDiameter(int n, int[][] edges) {
        // 首先，创建一个邻接表 adj 来表示给定的树。遍历输入的边，将边的两个顶点相互连接。
        List<Integer>[] adj = new List[n];
        for (int i = 0; i < n; i++) {
            adj[i] = new ArrayList<Integer>();
        }
        for (int[] edge : edges) {
            int x = edge[0] - 1;
            int y = edge[1] - 1;
            adj[x].add(y);
            adj[y].add(x);
        }

        // 初始化一个大小为 n-1 的数组 ans，用于保存每个直径的计数。
        int[] ans = new int[n - 1]; 

        // 使用一个整数 i 表示子树中包含的顶点。
        // 循环遍历 i 的所有可能值（从 1 到 (1 << n) - 1）。
        for (int i = 1; i < (1 << n); i++) {
            // 对于每个 i，找到它的最高有效位作为搜索根节点，然后执行深度优先搜索。
            int root = 32 - Integer.numberOfLeadingZeros(i) - 1;
            mask = i;
            diameter = 0;
            dfs(root, adj);
            // 如果搜索结束后，mask 的值为0（表示所有顶点都已访问过）(该子树为该无向图中的一个连通单元)
            // 且直径大于0，则 ans[diameter - 1] 的计数加1。
            if (mask == 0 && diameter > 0) {
                ans[diameter - 1]++;
            }
        }
        return ans;
    }

    public int dfs(int root, List<Integer>[] adj) {
        // 首先初始化两个变量 first 和 second，表示从根节点出发到达的最长和次长距离。
        int first = 0, second = 0;
        
        // 在 C++ 和 Java 中，~ 运算符是按位取反运算符。它会对一个整数的每个二进制位执行取反操作，将 0 变为 1，将 1 变为 0。
        // 在这段代码中，mask 是一个整数，用于表示子树中包含的顶点。
        // 每个二进制位表示一个顶点是否在子树中：如果某个位为 1，则相应的顶点在子树中；如果为 0，则不在。
        // 1 << root 是一个将 1 向左移动 root 位的位移操作。
        // 这将创建一个只有第 root 位为 1 的二进制数，其他位都为 0。
        // 例如，如果 root 等于 3，则 1 << root 将产生二进制数 00001000（十进制中的 8）。
        // ~(1 << root) 将对该二进制数执行按位取反操作。在上面的例子中，~(1 << root) 将产生二进制数 11110111。
        // mask &= ~(1 << root) 将 mask 和 ~(1 << root) 进行按位与（&）操作。
        // 这将把 mask 中第 root 位设置为 0，表示将顶点 root 从子树中移除。其他位保持不变，因为它们与 1 进行按位与操作时保持原值。
        // 使用按位操作将顶点 root 从当前子树的 mask 中移除，表示 root 已被访问。
        mask &= ~(1 << root);
        for (int vertex : adj[root]) {
            // a. 判断顶点 vertex 是否在子树中（由 mask 表示）。如果顶点 vertex 在子树中，执行以下操作：
            if ((mask & (1 << vertex)) != 0) {
                // 使用按位操作将顶点 vertex 从当前子树的 mask 中移除，表示 vertex 已被访问。
                mask &= ~(1 << vertex);

                // 对顶点 vertex 进行递归调用 dfs 函数，并将返回的距离加 1，得到从根节点 root 到顶点 vertex 的距离。
                int distance = 1 + dfs(vertex, adj);

                // 根据计算得到的距离，更新 first 和 second 的值。
                // 如果距离大于 first，则将 first 的值赋给 second，并将当前距离赋给 first。
                // 如果距离大于 second 但小于等于 first，则将当前距离赋给 second。
                if (distance > first) {
                    second = first;
                    first = distance;
                } else if (distance > second) {
                    second = distance;
                }
            }
        }

        // 遍历完所有相邻顶点后，更新子树的直径为 first 和 second 之和的最大值，即 Math.max(diameter, first + second)。
        diameter = Math.max(diameter, first + second);

        // 返回 first 作为搜索结果，即从根节点 root 出发到达的最长距离
        return first;
    }
}
// 复杂度分析 O(n x 2**n)

