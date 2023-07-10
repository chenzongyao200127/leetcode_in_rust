# 127_Word_Ladder
# https://leetcode.cn/problems/word-ladder/

# 字典 wordList 中从单词 beginWord 和 endWord 的 转换序列 是一个按下述规格形成的序列 beginWord -> s1 -> s2 -> ... -> sk：

# 每一对相邻的单词只差一个字母。
# 对于 1 <= i <= k 时，每个 si 都在 wordList 中。注意， beginWord 不需要在 wordList 中。
# sk == endWord
# 给你两个单词 beginWord 和 endWord 和一个字典 wordList ，返回 从 beginWord 到 endWord 的 最短转换序列 中的 单词数目 。
# 如果不存在这样的转换序列，返回 0 。

# 输入：beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
# 输出：5
# 解释：一个最短转换序列是 "hit" -> "hot" -> "dot" -> "dog" -> "cog", 返回它的长度 5。
# 示例 2：

# 输入：beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
# 输出：0
# 解释：endWord "cog" 不在字典中，所以无法进行转换。

# 超时
# from typing import List
# from pprint import pprint
# from collections import deque

# def ladderLength(beginWord: str, endWord: str, wordList: List[str]) -> int:
#     if endWord not in wordList:
#         return 0
#     def levenshtein_distance(word1: str, word2: str) -> int:
#         size_x = len(word1) + 1
#         size_y = len(word2) + 1
#         matrix = [[0] * size_y for _ in range(size_x)]
#         for x in range(size_x):
#             matrix [x][0] = x
#         for y in range(size_y):
#             matrix [0][y] = y

#         for x in range(1, size_x):
#             for y in range(1, size_y):
#                 if word1[x-1] == word2[y-1]:
#                     matrix [x][y] = min(
#                         matrix[x-1][y] + 1,
#                         matrix[x-1][y-1],
#                         matrix[x][y-1] + 1
#                     )
#                 else:
#                     matrix [x][y] = min(
#                         matrix[x-1][y] + 1,
#                         matrix[x-1][y-1] + 1,
#                         matrix[x][y-1] + 1
#                     )
#         return matrix[size_x - 1][size_y - 1]

#     map = [[0] * (len(wordList) + 1) for _ in range(len(wordList) + 1)]
#     pprint(map)
#     for i in range(len(wordList)):
#         if levenshtein_distance(beginWord, wordList[i]) == 1:
#             map[0][i+1] = 1
#     for i in range(len(wordList)-1):
#         for j in range(i+1, len(wordList)):
#             if levenshtein_distance(wordList[i], wordList[j]) == 1:
#                 map[i+1][j+1] = 1
#     pprint(map)
#     queue = deque([(0, 1)])
#     visited = set("0")
#     ans = float("inf")
#     while queue:
#         n, l = queue.popleft()
#         if n != 0 and wordList[n-1] == endWord:
#             print(n, l)
#             ans = min(ans, l)
#         for i in range(len(wordList) + 1):
#             if (map[n][i] == 1 or map[i][n] == 1) and str(i) not in visited:
#                 queue.append([i, l + 1])
#                 visited.add(str(i))
            
#     if ans == float("inf"):
#         return 0
#     else:
#         return ans
        
    

# print(ladderLength(beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]))



# 该代码的运行速度可以通过以下几种方式进行优化：

# 使用单词转换的规则：从起始单词开始，每次更改一个字母以形成新单词。对于字典中的每个单词，
# 生成所有可能的变异，并将其存储在映射中。这样可以在O(1)的时间内找到所有可能的下一步变化。

# 为了找到最短的转换序列，应该使用广度优先搜索（BFS）。与深度优先搜索（DFS）相比，BFS在这种情况下更有效，
# 因为它一次检查所有的一步变化，然后是所有的二步变化，等等。因此，一旦找到了转换序列，就可以确保它是最短的。

from collections import defaultdict, deque
from typing import List
from pprint import pprint


def ladderLength(beginWord: str, endWord: str, wordList: List[str]) -> int:
    if endWord not in wordList:
        return 0

    L = len(beginWord)

    all_combo_dict = defaultdict(list)

    for word in wordList:
        for i in range(L):
            all_combo_dict[word[:i] + "*" + word[i+1:]].append(word)
            
    pprint(all_combo_dict)

    queue = deque([(beginWord, 1)])
    visited = {beginWord: True}

    while queue:
        current_word, level = queue.popleft()
        for i in range(L):
            intermediate_word = current_word[:i] + "*" + current_word[i+1:]
            for word in all_combo_dict[intermediate_word]:
                if word == endWord:
                    return level + 1
                if word not in visited:
                    visited[word] = True
                    queue.append((word, level + 1))
            all_combo_dict[intermediate_word] = []
    return 0

print(ladderLength(beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]))



# 方法一：广度优先搜索 + 优化建图
# 本题要求的是最短转换序列的长度，看到最短首先想到的就是广度优先搜索。
# 想到广度优先搜索自然而然的就能想到图，但是本题并没有直截了当的给出图的模型，因此我们需要把它抽象成图的模型。

# 我们可以把每个单词都抽象为一个点，如果两个单词可以只改变一个字母进行转换，那么说明他们之间有一条双向边。
# 因此我们只需要把满足转换条件的点相连，就形成了一张图。

# 基于该图，我们以 beginWord 为图的起点，以 endWord 为终点进行广度优先搜索，寻找 beginWord 到 endWord 的最短路径。

# 基于上面的思路我们考虑如何编程实现。

# 首先为了方便表示，我们先给每一个单词标号，即给每个单词分配一个 id。
# 创建一个由单词 word 到 id 对应的映射 wordId，并将 beginWord 与 wordList 中所有的单词都加入这个映射中。
# 之后我们检查 endWord 是否在该映射内，若不存在，则输入无解。我们可以使用哈希表实现上面的映射关系。

# 然后我们需要建图，依据朴素的思路，我们可以枚举每一对单词的组合，判断它们是否恰好相差一个字符，以判断这两个单词对应的节点是否能够相连。
# 但是这样效率太低，我们可以优化建图。

# 具体地，我们可以创建虚拟节点。对于单词 hit，我们创建三个虚拟节点 *it、h*t、hi*，并让 hit 向这三个虚拟节点分别连一条边即可。
# 如果一个单词能够转化为 hit，那么该单词必然会连接到这三个虚拟节点之一。
# 对于每一个单词，我们枚举它连接到的虚拟节点，把该单词对应的 id 与这些虚拟节点对应的 id 相连即可。

# 最后我们将起点加入队列开始广度优先搜索，当搜索到终点时，我们就找到了最短路径的长度。
# 注意因为添加了虚拟节点，所以我们得到的距离为实际最短路径长度的两倍。
# 同时我们并未计算起点对答案的贡献，所以我们应当返回距离的一半再加一的结果。

# 方法二：双向广度优先搜索
# 思路及解法

# 根据给定字典构造的图可能会很大，而广度优先搜索的搜索空间大小依赖于每层节点的分支数量。
# 假如每个节点的分支数量相同，搜索空间会随着层数的增长指数级的增加。
# 考虑一个简单的二叉树，每一层都是满二叉树的扩展，节点的数量会以 2 为底数呈指数增长。

# 如果使用两个同时进行的广搜可以有效地减少搜索空间。
# 一边从 beginWord 开始，另一边从 endWord 开始。
# 我们每次从两边各扩展一层节点，当发现某一时刻两边都访问过同一顶点时就停止搜索。
# 这就是双向广度优先搜索，它可以可观地减少搜索空间大小，从而提高代码运行效率。

import collections
class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        def addWord(word: str):
            if word not in wordId:
                nonlocal nodeNum
                wordId[word] = nodeNum
                nodeNum += 1
        
        def addEdge(word: str):
            addWord(word)
            id1 = wordId[word]
            chars = list(word)
            for i in range(len(chars)):
                tmp = chars[i]
                chars[i] = "*"
                newWord = "".join(chars)
                addWord(newWord)
                id2 = wordId[newWord]
                edge[id1].append(id2)
                edge[id2].append(id1)
                chars[i] = tmp

        wordId = dict()
        edge = collections.defaultdict(list)
        nodeNum = 0

        for word in wordList:
            addEdge(word)
        
        addEdge(beginWord)
        if endWord not in wordId:
            return 0
        
        disBegin = [float("inf")] * nodeNum
        beginId = wordId[beginWord]
        disBegin[beginId] = 0
        queBegin = collections.deque([beginId])

        disEnd = [float("inf")] * nodeNum
        endId = wordId[endWord]
        disEnd[endId] = 0
        queEnd = collections.deque([endId])

        while queBegin or queEnd:
            queBeginSize = len(queBegin)
            for _ in range(queBeginSize):
                nodeBegin = queBegin.popleft()
                if disEnd[nodeBegin] != float("inf"):
                    return (disBegin[nodeBegin] + disEnd[nodeBegin]) // 2 + 1
                for it in edge[nodeBegin]:
                    if disBegin[it] == float("inf"):
                        disBegin[it] = disBegin[nodeBegin] + 1
                        queBegin.append(it)

            queEndSize = len(queEnd)
            for _ in range(queEndSize):
                nodeEnd = queEnd.popleft()
                if disBegin[nodeEnd] != float("inf"):
                    return (disBegin[nodeEnd] + disEnd[nodeEnd]) // 2 + 1
                for it in edge[nodeEnd]:
                    if disEnd[it] == float("inf"):
                        disEnd[it] = disEnd[nodeEnd] + 1
                        queEnd.append(it)
        
        return 0
    
    
# 具体地，我们可以创建虚拟节点。对于单词 hit，我们创建三个虚拟节点 *it、h*t、hi*，并让 hit 向这三个虚拟节点分别连一条边即可。
# 如果一个单词能够转化为 hit，那么该单词必然会连接到这三个虚拟节点之一。
# 对于每一个单词，我们枚举它连接到的虚拟节点，把该单词对应的 id 与这些虚拟节点对应的 id 相连即可。

# 最后我们将起点加入队列开始广度优先搜索，当搜索到终点时，我们就找到了最短路径的长度。
# 注意因为添加了虚拟节点，所以我们得到的距离为实际最短路径长度的两倍。
# 同时我们并未计算起点对答案的贡献，所以我们应当返回距离的一半再加一的结果。