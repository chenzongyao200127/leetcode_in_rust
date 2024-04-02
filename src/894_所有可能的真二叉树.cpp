#include <memory>
#include <vector>

using namespace std;

// 使用智能指针定义 TreeNode，以便自动管理内存
struct TreeNode
{
    int val;
    shared_ptr<TreeNode> left;
    shared_ptr<TreeNode> right;

    TreeNode(int x = 0, shared_ptr<TreeNode> left = nullptr, shared_ptr<TreeNode> right = nullptr)
        : val(x), left(left), right(right) {}
};

// 预分配向量以避免动态调整大小
vector<shared_ptr<TreeNode>> f[11];

auto init = []
{
    f[1].push_back(make_shared<TreeNode>()); // 使用 make_shared 以提高效率
    for (int i = 2; i < 11; i++)
    {
        f[i].reserve(estimateSize(i)); // 估算大小并预留空间以避免重复分配
        for (int j = 1; j < i; j++)
        {
            for (const auto &left : f[j])
            { // 使用引用避免不必要的智能指针复制
                for (const auto &right : f[i - j])
                {
                    f[i].push_back(make_shared<TreeNode>(0, left, right));
                }
            }
        }
    }
    return 0;
}();

// 估算具有 i 个节点的完全二叉树的数量
int estimateSize(int i)
{
    // 这个函数应该根据问题的限制提供一个估算
    // 这里是实际估算逻辑的占位符
    return 0;
}

class Solution
{
public:
    vector<shared_ptr<TreeNode>> allPossibleFBT(int n)
    {
        // 只有奇数节点的树才可能是完全二叉树
        if (n % 2 == 0)
            return {};
        // 提前计算索引一次
        int index = (n + 1) / 2;
        return f[index];
    }
};