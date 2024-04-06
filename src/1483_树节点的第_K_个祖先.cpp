// 1483_树节点的第_K_个祖先
// https://leetcode.cn/problems/kth-ancestor-of-a-tree-node/description/

#include <vector>

using namespace std;

class TreeAncestor
{
    // 2D vector to store the ancestors at different levels.
    vector<vector<int>> ancestors;

public:
    // Constructor that preprocesses the ancestors.
    TreeAncestor(int n, vector<int> &parent)
    {
        // Calculate the maximum number of levels needed (log base 2 of n).
        int max_levels = 32 - __builtin_clz(n);
        ancestors.resize(n, vector<int>(max_levels, -1));

        // Initialize the immediate parents.
        for (int i = 0; i < n; i++)
            ancestors[i][0] = parent[i];

        // Preprocess ancestors for each level.
        for (int level = 0; level < max_levels - 1; level++)
        {
            for (int node = 0; node < n; node++)
            {
                int immediate_parent = ancestors[node][level];
                if (immediate_parent != -1)
                {
                    ancestors[node][level + 1] = ancestors[immediate_parent][level];
                }
            }
        }
    }

    // Function to get the k-th ancestor of a node.
    int getKthAncestor(int node, int k)
    {
        int max_levels = 32 - __builtin_clz(k); // Binary length of k.

        // Traverse each bit of k.
        for (int i = 0; i < max_levels; i++)
        {
            if ((k >> i) & 1)
            { // Check if the i-th bit is set.
                node = ancestors[node][i];
                if (node < 0)
                    break; // If ancestor doesn't exist, exit early.
            }
        }
        return node;
    }

    // Alternative approach to get the k-th ancestor of a node.
    int getKthAncestor2(int node, int k)
    {
        // Loop until k is zero or there is no ancestor.
        while (k > 0 && node != -1)
        {
            // Move to the next ancestor.
            int level = __builtin_ctz(k); // Count the trailing zeros in k.
            node = ancestors[node][level];
            k &= k - 1; // Drop the lowest set bit of k.
        }
        return node;
    }
};
/**
 * Your TreeAncestor object will be instantiated and called as such:
 * TreeAncestor* obj = new TreeAncestor(n, parent);
 * int param_1 = obj->getKthAncestor(node,k);
 */