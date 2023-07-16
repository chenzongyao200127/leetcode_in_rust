// 126. 单词接龙 II
// https://leetcode.cn/problems/word-ladder-ii/


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
#include <unordered_set>
#include <limits.h>
using namespace std;
typedef long long ll;
const double PI = acos(-1.0);
const double eps = 1e-6;
const int INF = 0x3f3f3f3f;


// 按字典 wordList 完成从单词 beginWord 到单词 endWord 转化，一个表示此过程的 转换序列 
// 是形式上像 beginWord -> s1 -> s2 -> ... -> sk 这样的单词序列，并满足：

// 每对相邻的单词之间仅有单个字母不同。
// 转换过程中的每个单词 si（1 <= i <= k）必须是字典 wordList 中的单词。注意，beginWord 不必是字典 wordList 中的单词。
// sk == endWord
// 给你两个单词 beginWord 和 endWord ，以及一个字典 wordList 。
// 请你找出并返回所有从 beginWord 到 endWord 的 最短转换序列 ，如果不存在这样的转换序列，返回一个空列表。
// 每个序列都应该以单词列表 [beginWord, s1, s2, ..., sk] 的形式返回。

// 方法一：广度优先搜索 + 回溯
// 思路

// 本题要求的是最短转换序列，看到最短首先想到的就是广度优先搜索。
// 但是本题没有给出显示的图结构，根据单词转换规则：把每个单词都抽象为一个顶点，
// 如果两个单词可以只改变一个字母进行转换，那么说明它们之间有一条双向边。
// 因此我们只需要把满足转换条件的点相连，就形成了一张图。根据示例 1 中的输入，我们可以建出下图：



// 基于该图，我们以 “hit" 为图的起点， 以 “cog" 为终点进行广度优先搜索，寻找 hit" 到 cog 的最短路径。下图即为答案中的一条路径。

// 由于要求输出所有的最短路径，因此我们需要记录遍历路径，然后通过回溯得到所有的最短路径。

// 细节

// 从一个单词出发，修改每一位字符，将它修改成为 ‘a’ 到 ‘z’ 中的所有字符，看看修改以后是不是在题目中给出的单词列表中；
// 有一些边的关系，由于不是最短路径上的边，不可以被记录下来。为此，我们为扩展出的单词记录附加的属性：层数。
// 即下面代码中的 steps。如果当前的单词扩散出去得到的单词的层数在以前出现过，则不应该记录这样的边的关系。
// 其它细节我们放在「代码」中，细节的部分比较多，读者朋友们需要仔细调试，相信掌握这道题对于大家来说会是一个很不错的编程练习。

class Solution {
public:
    vector<vector<string>> findLadders(string beginWord, string endWord, vector<string> &wordList) {
        vector<vector<string>> res;
        // 因为需要快速判断扩展出的单词是否在 wordList 里，因此需要将 wordList 存入哈希表，这里命名为「字典」
        unordered_set<string> dict = {wordList.begin(), wordList.end()};
        // 修改以后看一下，如果根本就不在 dict 里面，跳过
        if (dict.find(endWord) == dict.end()) {
            return res;
        }
        // 特殊用例处理
        dict.erase(beginWord);

        // 第 1 步：广度优先搜索建图
        // 记录扩展出的单词是在第几次扩展的时候得到的，key：单词，value：在广度优先搜索的第几层
        unordered_map<string, int> steps = {{beginWord, 0}};
        // 记录了单词是从哪些单词扩展而来，key：单词，value：单词列表，这些单词可以变换到 key ，它们是一对多关系
        unordered_map<string, set<string>> from = {{beginWord, {}}};
        int step = 0;
        bool found = false;
        queue<string> q = queue<string>{{beginWord}};
        int wordLen = beginWord.length();
        while (!q.empty()) {
            step++;
            int size = q.size();
            for (int i = 0; i < size; i++) {
                const string currWord = move(q.front());
                string nextWord = currWord;
                q.pop();
                // 将每一位替换成 26 个小写英文字母
                for (int j = 0; j < wordLen; ++j) {
                    const char origin = nextWord[j];
                    for (char c = 'a'; c <= 'z'; ++c) {
                        nextWord[j] = c;
                        if (steps[nextWord] == step) {
                            from[nextWord].insert(currWord);
                        }
                        if (dict.find(nextWord) == dict.end()) {
                            continue;
                        }
                        // 如果从一个单词扩展出来的单词以前遍历过，距离一定更远，为了避免搜索到已经遍历到，且距离更远的单词，需要将它从 dict 中删除
                        dict.erase(nextWord);
                        // 这一层扩展出的单词进入队列
                        q.push(nextWord);
                        // 记录 nextWord 从 currWord 而来
                        from[nextWord].insert(currWord);
                        // 记录 nextWord 的 step
                        steps[nextWord] = step;
                        if (nextWord == endWord) {
                            found = true;
                        }
                    }
                    nextWord[j] = origin;
                }
            }
            if (found) {
                break;
            }
        }
        // 第 2 步：回溯找到所有解，从 endWord 恢复到 beginWord ，所以每次尝试操作 path 列表的头部
        if (found) {
            vector<string> Path = {endWord};
            backtrack(res, endWord, from, Path);
        }
        return res;
    }

    void backtrack(vector<vector<string>> &res, const string &Node, unordered_map<string, set<string>> &from,
             vector<string> &path) {
        if (from[Node].empty()) {
            res.push_back({path.rbegin(), path.rend()});
            return;
        }
        for (const string &Parent: from[Node]) {
            path.push_back(Parent);
            backtrack(res, Parent, from, path);
            path.pop_back();
        }
    }
};