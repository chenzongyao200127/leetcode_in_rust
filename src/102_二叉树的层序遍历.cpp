#include <vector>
#include <queue>

using namespace std;

// Definition for a binary tree node.
struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution
{
public:
    vector<vector<int>> levelOrder(TreeNode *root)
    {
        if (!root)
            return {}; // Return an empty vector if the tree is empty.

        vector<vector<int>> ans;
        queue<TreeNode *> q; // Queue that holds the nodes to process.
        q.push(root);        // Start with the root.

        while (!q.empty())
        {
            int layer_size = q.size(); // Number of nodes at the current level.
            vector<int> cur_layer;

            for (int i = 0; i < layer_size; ++i)
            {
                TreeNode *node = q.front();
                q.pop();
                cur_layer.push_back(node->val); // Add the current node's value to the layer vector.

                if (node->left)
                    q.push(node->left); // Add left child to queue if it exists.
                if (node->right)
                    q.push(node->right); // Add right child to queue if it exists.
            }

            ans.push_back(cur_layer); // Add the current level's values to the answer vector.
        }

        return ans;
    }
};