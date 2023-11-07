// 117_Populating_Next_Right_Pointers_in_Each_Node_II
// https://leetcode.cn/problems/populating-next-right-pointers-in-each-node-ii/?envType=daily-question&envId=2023-11-03


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
using namespace std;
typedef long long ll;
const double PI = acos(-1.0);
const double eps = 1e-6;
const int INF = 0x3f3f3f3f;
const int maxn = 100;
int T,n,m;
#include <unordered_map>
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <limits.h>

using namespace std;



// Definition for a Node.
class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() : val(0), left(NULL), right(NULL), next(NULL) {}

    Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {}

    Node(int _val, Node* _left, Node* _right, Node* _next)
        : val(_val), left(_left), right(_right), next(_next) {}
};

class Solution {
public:
    Node* connect(Node* root) {
        if (root == nullptr) {
            return nullptr; // If the tree is empty, return null immediately.
        }

        queue<Node*> nodesQueue;
        nodesQueue.push(root);

        // Process nodes level by level.
        while (!nodesQueue.empty()) {
            int levelSize = nodesQueue.size();
            Node* previousNode = nullptr;

            // Link all nodes on the current level.
            for (int i = 0; i < levelSize; ++i) {
                Node* currentNode = nodesQueue.front();
                nodesQueue.pop();

                // Add child nodes to the queue for the next level.
                if (currentNode->left) {
                    nodesQueue.push(currentNode->left);
                }
                if (currentNode->right) {
                    nodesQueue.push(currentNode->right);
                }

                // Link the previous node with the current node.
                if (previousNode) {
                    previousNode->next = currentNode;
                }
                previousNode = currentNode;
            }

            // The last node of each level points to null, which is already the default.
        }

        return root; // Return the root as it's now connected.
    }
};
