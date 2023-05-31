// 87_Scramble_String
// https://leetcode.cn/problems/scramble-string/

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
const int maxn = 100;
int T,n,m;
#include <unordered_map>
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <limits.h>

using namespace std;

// 记忆化搜索
class Solution {
public:
	vector<vector<vector<int>>> cache;
	string s1, s2;
	int n;
	int NO = -1, YES = 1, EMPTY = 0;

    bool isScramble(string _s1, string _s2) {
    	s1 = _s1, s2 = _s2;
    	if(s1 == s2) return true;
    	if(s1.size() != s2.size()) return false;
    	n = s1.size();
    	cache.resize(n, vector<vector<int>>(n, vector<int>(n+1, 0)));

    	return dfs(0, 0, n);
    }

    bool dfs(int i, int j, int len) {
    	if(cache[i][j][len] != EMPTY)
    		return cache[i][j][len] == YES;

    	string a = s1.substr(i, len), b = s2.substr(j, len);
    	
    	if(a == b)
    		return cache[i][j][len] = YES || true;

    	if(!check(a, b))
    		return cache[i][j][len] = NO && false;

    	for(int k = 1; k < len; k++) {
    		if(dfs(i, j, k) && dfs(i+k, j+k, len-k))
    			return cache[i][j][len] = YES || true;

    		if(dfs(i, j+len-k, k) && dfs(i+k, j, len-k))
    			return cache[i][j][len] = YES || true;
    	}

    	cache[i][j][len] = NO;
    	return false;
    }

    bool check(string s1, string s2) {
    	if(s1.size() != s2.size()) return false;
    	vector<int> cnt1(26,0), cnt2(26,0);
    	for(auto c : s1)
    		cnt1[c-'a']++;
    	for(auto c : s2)
    		cnt2[c-'a']++;
    	return cnt1 == cnt2;
    }
};