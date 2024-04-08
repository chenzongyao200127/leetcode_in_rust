// 最近公共祖先 模板
// https://leetcode.cn/problems/kth-ancestor-of-a-tree-node/solutions/2305895/mo-ban-jiang-jie-shu-shang-bei-zeng-suan-v3rw/

#include <queue>
#include <vector>
#include <functional>
#include <utility>

using namespace std;

class TreeAncestor
{
    vector<int> depth;          // 存储每个节点的深度
    vector<vector<int>> parent; // 祖先矩阵，parent[i][j] 表示节点 i 的第 2^j 级祖先

public:
    TreeAncestor(vector<pair<int, int>> &edges)
    {
        int nodeCount = edges.size() + 1;             // 计算节点数量
        int maxPower = 32 - __builtin_clz(nodeCount); // 计算二进制位数

        // 构建无向图，g 表示图的邻接表
        vector<vector<int>> graph(nodeCount);
        for (const auto &[from, to] : edges)
        {
            graph[from].push_back(to);
            graph[to].push_back(from);
        }

        depth.resize(nodeCount);
        parent.assign(nodeCount, vector<int>(maxPower, -1)); // 初始化祖先矩阵

        // 深度优先遍历来填充 depth 和 parent 矩阵
        function<void(int, int)> dfs = [&](int current, int father)
        {
            parent[current][0] = father;
            for (int neighbor : graph[current])
            {
                if (neighbor != father)
                {
                    depth[neighbor] = depth[current] + 1;
                    dfs(neighbor, current);
                }
            }
        };
        dfs(0, -1); // 假设根节点编号为 0

        // 动态规划填充更高层的祖先信息
        for (int i = 0; i < maxPower - 1; ++i)
        {
            for (int node = 0; node < nodeCount; ++node)
            {
                if (int ancestor = parent[node][i]; ancestor != -1)
                {
                    parent[node][i + 1] = parent[ancestor][i];
                }
            }
        }
    }

    // 获取节点 node 的第 k 级祖先节点
    int getKthAncestor(int node, int k)
    {
        while (k > 0)
        {
            int shift = __builtin_ctz(k);
            node = parent[node][shift];
            if (node == -1)
                break;
            k -= (1 << shift);
        }
        return node;
    }

    // 返回节点 x 和 y 的最近公共祖先
    int getLCA(int x, int y)
    {
        // 保证 y 的深度不小于 x
        if (depth[x] > depth[y])
        {
            swap(x, y);
        }

        // 将 y 提升到与 x 相同的深度
        y = getKthAncestor(y, depth[y] - depth[x]);
        if (y == x)
        { // 检查是否已经找到公共祖先
            return x;
        }

        // 一起向上提升 x 和 y
        for (int i = parent[x].size() - 1; i >= 0; --i)
        {
            if (parent[x][i] != parent[y][i])
            {
                x = parent[x][i];
                y = parent[y][i];
            }
        }

        // 返回最近公共祖先
        return parent[x][0];
    }
};

