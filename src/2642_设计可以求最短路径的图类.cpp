// 2642_设计可以求最短路径的图类
// https://leetcode.cn/problems/design-graph-with-shortest-path-calculator/description/
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <queue>   // Include for priority_queue
#include <climits> // Include for INT_MAX
using namespace std;

class Graph
{
public:
    using pii = pair<int, int>;
    Graph(int n, vector<vector<int>> &edges)
    {
        this->graph = vector<vector<pii>>(n);
        for (auto &vec : edges)
        {
            int x = vec[0];
            int y = vec[1];
            int cost = vec[2];
            graph[x].emplace_back(y, cost);
        }
    }

    void addEdge(vector<int> edge)
    {
        int x = edge[0];
        int y = edge[1];
        int cost = edge[2];
        graph[x].emplace_back(y, cost);
    }

    int shortestPath(int node1, int node2)
    {
        priority_queue<pii, vector<pii>, greater<pii>> pq;
        vector<int> dist(graph.size(), INT_MAX);
        dist[node1] = 0;
        pq.emplace(0, node1);
        while (!pq.empty())
        {
            auto [cost, cur] = pq.top();
            pq.pop();
            if (cur == node2)
            {
                return cost;
            }
            if (cost > dist[cur]) // Skip if this path is not the shortest
                continue;
            for (auto [next, ncost] : graph[cur])
            {
                if (dist[next] > dist[cur] + ncost)
                {
                    dist[next] = dist[cur] + ncost;
                    pq.emplace(dist[next], next);
                }
            }
        }
        return -1;
    }

private:
    vector<vector<pii>> graph;
};

// Additional usage code (e.g., main function) would go here