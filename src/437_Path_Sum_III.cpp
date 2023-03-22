// 437_Path_Sum_III
// https://leetcode.cn/problems/path-sum-iii/

#include <unordered_map>
using namespace std;

/**
 * Definition for a binary tree node.
 */
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
public:
    int rootSum(TreeNode* root, int targetSum) {
        if (!root) {
            return 0;
        }

        int ret = 0;
        if (root->val == targetSum) {
            ret++;
        } 

        ret += rootSum(root->left, targetSum - root->val);
        ret += rootSum(root->right, targetSum - root->val);
        return ret;
    }

    int pathSum(TreeNode* root, int targetSum) {
        if (!root) {
            return 0;
        }
        
        int ret = rootSum(root, targetSum);
        ret += pathSum(root->left, targetSum);
        ret += pathSum(root->right, targetSum);
        return ret;
    }
};


class Solution {
public:
    // // key是前缀和, value是大小为key的前缀和出现的次数
    unordered_map<long long, int> prefix;

    int dfs(TreeNode *root, long long curr, int targetSum) {
        // 1.递归终止条件
        if (!root) {
            return 0;
        }

        // 2.本层要做的事情
        int ret = 0;
        // 当前路径上的和
        curr += root->val;

        //---核心代码
        // 看看root到当前节点这条路上是否存在节点前缀和加target为currSum的路径
        // 当前节点->root节点反推，有且仅有一条路径，如果此前有和为currSum-target,而当前的和又为currSum,两者的差就肯定为target了
        // currSum-target相当于找路径的起点，起点的sum+target=currSum，当前点到起点的距离就是target
        if (prefix.count(curr - targetSum)) {
            ret = prefix[curr - targetSum];
        }
        // 更新路径上当前节点前缀和的个数
        prefix[curr]++;
        //---核心代码

        // 3.进入下一层
        ret += dfs(root->left, curr, targetSum);
        ret += dfs(root->right, curr, targetSum);

        // 4.回到本层，恢复状态，去除当前节点的前缀和数量
        prefix[curr]--;

        return ret;
    }

    int pathSum(TreeNode* root, int targetSum) {
        prefix[0] = 1;

        // 前缀和的递归回溯思路
        return dfs(root, 0, targetSum);
    }
};

// = 【 (路径包括本节点的情况) 左节点 + 右节点 】 + 【(不包括之前所有节点) 左节点 + 右节点】+ 当前是否为符合路径
class Solution {
public:
    int pathSum(TreeNode* root, int targetSum, bool k = true) {
        return root ? 
        pathSum(root->left, targetSum-root->val, false) + pathSum(root->right, targetSum-root->val, false) + 
        (k ? pathSum(root->left, targetSum, true) + pathSum(root->right, targetSum, true) : 0) + 
        (targetSum == root->val ? 1 : 0) 
        : 0;
    }
};