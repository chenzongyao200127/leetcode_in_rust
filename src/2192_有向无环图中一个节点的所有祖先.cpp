// 2192_有向无环图中一个节点的所有祖先
// https://leetcode.cn/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/description/
#include <vector>
#include <queue>
#include <unordered_set>
#include <algorithm>

using namespace std;

class Solution
{
public:
    vector<vector<int>> getAncestors(int n, vector<vector<int>> &edges)
    {
        vector<vector<int>> ans(n);
        vector<int> indegrees(n, 0);
        vector<unordered_set<int>> rg(n);

        // construct reverse graph
        for (auto &edge : edges)
        {
            int from = edge[0];
            int to = edge[1];
            rg[to].insert(from);
            indegrees[from]++;
        }

        // find ancestors for each node
        for (int i = 0; i < n; i++)
        {
            queue<int> q;
            unordered_set<int> visited;

            q.push(i);
            visited.insert(i);

            while (!q.empty())
            {
                int cur = q.front();
                q.pop();

                for (int ancestor : rg[cur])
                {
                    if (visited.count(ancestor) == 0)
                    {
                        ans[i].push_back(ancestor);
                        q.push(ancestor);
                        visited.insert(ancestor);
                    }
                }
            }

            // Sort and remove duplicates as required
            sort(ans[i].begin(), ans[i].end());
            ans[i].erase(unique(ans[i].begin(), ans[i].end()), ans[i].end());
        }

        return ans;
    }
};
