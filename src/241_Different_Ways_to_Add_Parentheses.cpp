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

vector<int> diffWaysToCompute(string input) {
    vector<int> res;
    for(int i = 0; i < input.size(); ++i){
        char c = input[i];
        if(c == '+'|| c == '-' || c == '*'){ //递归划分
            auto res1 = diffWaysToCompute(input.substr(0, i));
            auto res2 = diffWaysToCompute(input.substr(i+1));
            for(auto r1 : res1){ //计算结果
                for(auto r2 : res2){
                    if(c == '+'){
                        res.push_back(r1 + r2);
                    } else if(c == '-'){
                        res.push_back(r1 - r2);
                    } else if(c == '*'){
                        res.push_back(r1 * r2);
                    }
                }
            }
        }
    }
    if(res.empty()){ //纯数字，input中没符号
        res.push_back(stoi(input));
    }   
    return res;
}