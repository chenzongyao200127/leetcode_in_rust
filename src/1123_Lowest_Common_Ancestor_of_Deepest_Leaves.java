// 1123_Lowest_Common_Ancestor_of_Deepest_Leaves
// https://leetcode.cn/problems/lowest-common-ancestor-of-deepest-leaves/description/

// 给你一个有根节点 root 的二叉树，返回它 最深的叶节点的最近公共祖先 。

// 回想一下：

// 叶节点 是二叉树中没有子节点的节点
// 树的根节点的 深度 为 0，如果某一节点的深度为 d，那它的子节点的深度就是 d+1
// 如果我们假定 A 是一组节点 S 的 最近公共祖先，
// S 中的每个节点都在以 A 为根节点的子树中，且 A 的深度达到此条件下可能的最大值。

/**
 * Definition for a binary tree node.
 */
class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;
    TreeNode(int x) { val = x; }
}

class Solution {
    public TreeNode subtreeWithAllDeepest(TreeNode root) {
        return dfs(root).subtree;
    }

    private class Result {
        int depth;
        TreeNode subtree;
        Result(int depth, TreeNode subtree) {
            this.depth = depth;
            this.subtree = subtree;
        }
    }

    private Result dfs(TreeNode node) {
        if (node == null) {
            return new Result(0, null);
        }

        Result leftResult = dfs(node.left);
        Result rightResult = dfs(node.right);

        if (leftResult.depth == rightResult.depth) {
            return new Result(leftResult.depth + 1, node);
        } else if (leftResult.depth > rightResult.depth) {
            return new Result(leftResult.depth + 1, leftResult.subtree);
        } else {
            return new Result(rightResult.depth + 1, rightResult.subtree);
        }
    }
}
