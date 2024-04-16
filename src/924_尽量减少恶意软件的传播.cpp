// 924_尽量减少恶意软件的传播
// https://leetcode.cn/problems/minimize-malware-spread/description/

#include <vector>
#include <algorithm>
#include <unordered_map>
#include <limits.h>

using namespace std;

class UnionFind
{
private:
    vector<int> parent, rank, size;

public:
    UnionFind(int n) : parent(n), rank(n, 0), size(n, 1)
    {
        for (int i = 0; i < n; ++i)
        {
            parent[i] = i;
        }
    }

    int find(int x)
    {
        if (parent[x] != x)
        {
            parent[x] = find(parent[x]); // Path compression
        }
        return parent[x];
    }

    void unionSets(int x, int y)
    {
        int rootX = find(x);
        int rootY = find(y);

        if (rootX != rootY)
        {
            if (rank[rootX] > rank[rootY])
            {
                parent[rootY] = rootX;
                size[rootX] += size[rootY];
            }
            else if (rank[rootX] < rank[rootY])
            {
                parent[rootX] = rootY;
                size[rootY] += size[rootX];
            }
            else
            {
                parent[rootY] = rootX;
                rank[rootX]++;
                size[rootX] += size[rootY];
            }
        }
    }

    int getSize(int x)
    {
        return size[find(x)];
    }
};

class Solution
{
public:
    int minMalwareSpread(vector<vector<int>> &graph, vector<int> &initial)
    {
        int n = graph.size();
        UnionFind uf(n);

        // Build the union find structure
        for (int i = 0; i < n; ++i)
        {
            for (int j = i + 1; j < n; ++j)
            {
                if (graph[i][j] == 1)
                {
                    uf.unionSets(i, j);
                }
            }
        }

        // Count the number of infected nodes in each component
        unordered_map<int, int> componentInfectedCount;
        for (int node : initial)
        {
            int root = uf.find(node);
            componentInfectedCount[root]++;
        }

        // Determine the best node to remove
        sort(initial.begin(), initial.end());
        int minAffected = INT_MAX, result = -1;

        for (int node : initial)
        {
            int root = uf.find(node);
            int affected = 0;

            if (componentInfectedCount[root] == 1)
            {
                // Only consider if this node is the only infection in its component
                affected = uf.getSize(root);
            }

            if (affected < minAffected)
            {
                minAffected = affected;
                result = node;
            }
        }

        return result;
    }
};