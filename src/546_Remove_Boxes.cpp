// 546. Remove Boxes
// https://leetcode.cn/problems/remove-boxes/description/


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
using namespace std;
typedef long long ll;
const double PI = acos(-1.0);
const double eps = 1e-6;
const int INF = 0x3f3f3f3f;

class Solution {
public:
    // dp数组用于存储子问题的结果。维度是100x100x100，用于存储三个参数的子问题。
    int dp[100][100][100];

    int removeBoxes(vector<int>& boxes) {
        // 将dp数组初始化为0
        memset(dp, 0, sizeof dp);
        // 调用辅助函数calculatePoints来计算最大分数
        return calculatePoints(boxes, 0, boxes.size() - 1, 0);
    }

    int calculatePoints(vector<int>& boxes, int l, int r, int k) {
        // 基础情况：如果左边界大于右边界，则返回0
        if (l > r) {
            return 0;
        }

        // 如果当前子问题的解还未被计算
        if (dp[l][r][k] == 0) {
            // 默认情况：移除右边界的盒子并获得(k+1)*(k+1)的分数
            dp[l][r][k] = calculatePoints(boxes, l, r - 1, 0) + (k + 1) * (k + 1);

            // 遍历当前子数组，寻找和右边界盒子颜色相同的盒子，看是否有机会合并
            for (int i = l; i < r; i++) {
                if (boxes[i] == boxes[r]) {
                    // 尝试合并和右边界相同颜色的盒子，并计算两个子问题：一个是[l, i]范围内的盒子，另一个是[i+1, r-1]范围内的盒子
                    dp[l][r][k] = max(dp[l][r][k], calculatePoints(boxes, l, i, k + 1) + calculatePoints(boxes, i + 1, r - 1, 0));
                }
            }
        }
        // 返回当前子问题的解
        return dp[l][r][k];
    }
};
