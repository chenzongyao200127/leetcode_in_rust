// 1483_Kth_Ancestor_of_a_Tree_Node
// https://leetcode.cn/problems/kth-ancestor-of-a-tree-node/

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


class TreeAncestor {
    vector<vector<int>> pa;
public:
    TreeAncestor(int n, vector<int> &parent) {
        int m = 32 - __builtin_clz(n); // n 的二进制长度
        pa.resize(n, vector<int>(m, -1));
        for (int i = 0; i < n; i++)
            pa[i][0] = parent[i];
        for (int i = 0; i < m - 1; i++)
            for (int x = 0; x < n; x++)
                if (int p = pa[x][i]; p != -1)
                    pa[x][i + 1] = pa[p][i];
    }

    int getKthAncestor(int node, int k) {
        int m = 32 - __builtin_clz(k); // k 的二进制长度
        for (int i = 0; i < m; i++) {
            if ((k >> i) & 1) { // k 的二进制从低到高第 i 位是 1
                node = pa[node][i];
                if (node < 0) break;
            }
        }
        return node;
    }

    // 另一种写法，不断去掉 k 的最低位的 1
    int getKthAncestor2(int node, int k) {
        for (; k && node != -1; k &= k - 1) // 也可以写成 ~node
            node = pa[node][__builtin_ctz(k)];
        return node;
    }
};

