#include<cstdio>
#include<cstring>
#include<algorithm>
#include<iostream>
#include<string>
#include<vector>
#include<stack>
#include<bitset>
#include<cstdlib>
#include<cmath>
#include<set>
#include<list>
#include<deque>
#include<map>
#include<queue>
#include <unordered_map>
#include <limits.h>
using namespace std;
typedef long long ll;
const double PI = acos(-1.0);
const double eps = 1e-6;
const int INF = 0x3f3f3f3f;

class Solution {
public:
    // 定义四个方向，分别为上下左右
    int dirs[4][2] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

    // 广度优先搜索函数，寻找从(sx, sy)到(tx, ty)的最短路径
    int bfs(vector<vector<int>>& forest, int sx, int sy, int tx, int ty) {
        if (sx == tx && sy == ty) {
            return 0;  // 如果起始位置和目标位置相同，直接返回0
        }

        int row = forest.size();  // 矩阵行数
        int col = forest[0].size();  // 矩阵列数
        priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> pq;  // 定义优先队列
        vector<vector<bool>> visited(row, vector<bool>(col, false));  // 定义标记数组，标记某位置是否被访问过
        pq.emplace(0, sx * col + sy);  // 将起始位置入队
        visited[sx][sy] = true;  // 将起始位置标记为已访问
        while (!pq.empty()) {
            auto p = pq.top();
            auto dist = p.first;
            auto loc = p.second;
            pq.pop();
            for (int j = 0; j < 4; ++j) {  // 遍历四个方向
                int nx = loc / col + dirs[j][0];  // 计算新位置的行
                int ny = loc % col + dirs[j][1];  // 计算新位置的列
                if (nx >= 0 && nx < row && ny >= 0 && ny < col) {  // 判断新位置是否有效
                    if (!visited[nx][ny] && forest[nx][ny] > 0) {  // 判断新位置是否已被访问过，以及新位置是否可行走
                        if (nx == tx && ny == ty) {
                            return dist + 1;  // 如果新位置是目标位置，直接返回当前距离+1
                        }
                        pq.emplace(dist + 1, nx * col + ny);  // 将新位置入队
                        visited[nx][ny] = true;  // 标记新位置为已访问
                    }
                }
            }
        }
        return -1;  // 如果找不到路径，返回-1
    }

    // 主函数
    int cutOffTree(vector<vector<int>>& forest) {
        vector<pair<int, int>> trees;  // 定义一个存储所有树的数组
        int row = forest.size();  // 矩阵行数
        int col = forest[0].size();  // 矩阵列数
        for (int i = 0; i < row; ++i) {
            for (int j = 0; j < col; ++j) {
                if (forest[i][j] > 1) {
                    trees.emplace_back(i, j);  // 如果当前位置是一棵树，将其加入数组
                }
            }
        }
        // 按照树的高度排序
        sort(trees.begin(), trees.end(), [&](const pair<int, int> & a, const pair<int, int> & b) {
            return forest[a.first][a.second] < forest[b.first][b.second];
        });
        
        int cx = 0;
        int cy = 0;
        int ans = 0;
        for (int i = 0; i < trees.size(); ++i) {
            int steps = bfs(forest, cx, cy, trees[i].first, trees[i].second);  // 寻找从当前位置到下一棵树的最短路径
            if (steps == -1) {
                return -1;  // 如果找不到路径，返回-1
            }
            ans += steps;  // 将路径长度累加到结果中
            cx = trees[i].first;  // 更新当前位置
            cy = trees[i].second;
        }
        return ans;  // 返回结果
    }
};
