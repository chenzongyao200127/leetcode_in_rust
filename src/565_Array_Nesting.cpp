// 565_Array_Nesting
// https://leetcode.cn/problems/array-nesting/

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
    int arrayNesting(vector<int> &nums) {
        int ans = 0;
        int n = nums.size();
        vector<int> vis(n);
        for (int i = 0; i < n; ++i) {
            int cnt = 0;
            while (!vis[i]) {
                vis[i] = true;
                i = nums[i];
                ++cnt;
            }
            ans = max(ans, cnt);
        }

        return ans;
    }
};