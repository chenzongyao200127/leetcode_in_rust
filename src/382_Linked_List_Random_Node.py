# 382_Linked_List_Random_Node
# https://leetcode.cn/problems/linked-list-random-node/description/

# 蓄水池抽样（Reservoir Sampling）是一种用于从一组数据中随机选择k个样本的算法，尤其适用于在不知道数据集总体大小的情况下进行样本抽取。这种情况常见于数据流或大型数据集中，其中数据量可能非常庞大或无法一次性全部加载到内存中。

# ### 基本原理

# 蓄水池抽样的基本思想是保持一个大小为k的“蓄水池”，该蓄水池在整个过程中保持样本的随机性。算法流程大致如下：

# 1. **初始化蓄水池**：将数据流中的前k个元素放入蓄水池中。
# 2. **处理剩余元素**：对于每一个新的元素（第k+1个、k+2个...），以一定的概率选择该元素替换蓄水池中的某个元素。这个概率是根据已处理的元素数量来确定的。

# ### 算法步骤

# 对于数据流中的第i个元素（i > k）：

# 1. 以k/i的概率选择这个新元素。
# 2. 如果选择了这个新元素，则随机选择蓄水池中的一个元素并用新元素替换它。

# ### 重要特性

# - **公平性**：蓄水池抽样算法保证了每个元素被选中的概率是相等的。
# - **适用于大数据和数据流**：由于不需要一开始就知道数据的总量，这个算法特别适用于数据流和大规模数据集的场景。

# ### 应用实例

# 一个典型的应用场景是从一个非常大的数据集中随机抽取样本，这个数据集可能是动态生成的，例如实时的网络数据流，或者是如此之大以至于无法完全加载到内存中。

# ### Python实现示例

# 以下是一个简单的Python实现示例，用于从一个未知大小的数据流中抽取k个样本：


import random

def reservoir_sampling(stream, k):
    reservoir = []
    for i, item in enumerate(stream):
        if i < k:
            reservoir.append(item)
        else:
            # Randomly replace elements in the reservoir with a decreasing probability.
            j = random.randint(0, i)
            if j < k:
                reservoir[j] = item
    return reservoir


# 在这个实现中，`stream`代表数据流，而`k`是蓄水池的大小。

from typing import Optional
from random import randint

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:

    def __init__(self, head: Optional[ListNode]):
        self.root = head
        

    def getRandom(self) -> int:
        node, ans, idx = self.root, 0
        while node:
            if not randint(0, idx):
                ans = node.val
            node = node.next
            idx += 1
        return ans

    
# 蓄水池抽样是一种在不完全了解整个数据集（比如数据集非常大或者数据是实时流入的）的情况下，从中随机抽取一定数量样本的方法。
# 想象一下，你有一个蓄水池，它只能装下固定数量的水，这就像你只能从大量数据中选择固定数量的样本。

# ### 如何理解蓄水池抽样？

# 假设你有一个非常大的湖泊（数据集），但你只有一个小桶（蓄水池），这个小桶只能装k桶水。现在，你的任务是确保这k桶水能代表整个湖泊的水。

# 1. **开始装水**：首先，你从湖中随便舀k桶水装满你的小桶。这很简单，因为你的桶刚开始是空的。

# 2. **继续舀水**：然后，你继续舀湖里的水，但现在每次你舀一桶水时，你都要做一个决定：是把这桶水倒掉，还是用它来替换桶里的某桶水。这个决定不是随机的，而是有一定规则的。比如，当你舀第10桶水时，你有10%的几率用这桶水替换桶里的任意一桶水；当舀第100桶水时，你有1%的几率替换，依此类推。这样做的目的是确保每桶湖水都有平等的机会被选中。

# 3. **最终结果**：随着你不断地舀水，桶里的水会不断更新，但总是保持着k桶。最终，当你决定停止舀水时，桶里的这k桶水就是你从整个湖泊中随机抽取的样本。

# ### 为什么要这样做？

# 蓄水池抽样的好处是它允许你在不知道数据集大小的情况下，或者当数据集太大以至于无法一次性处理时，仍然能够随机抽取样本。这在处理大数据或实时数据流时特别有用。

# Your Solution object will be instantiated and called as such:
# obj = Solution(head)
# param_1 = obj.getRandom()

# 如果链表非常大且长度未知，该怎么处理？
# 你能否在不使用额外空间的情况下解决此问题？
