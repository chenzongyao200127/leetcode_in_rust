# 126_Word_Ladder_II
# https://leetcode.cn/problems/word-ladder-ii/

# 按字典 wordList 完成从单词 beginWord 到单词 endWord 转化，一个表示此过程的 转换序列
# 是形式上像 beginWord -> s1 -> s2 -> ... -> sk 这样的单词序列，并满足：

# 每对相邻的单词之间仅有单个字母不同。
# 转换过程中的每个单词 si（1 <= i <= k）必须是字典 wordList 中的单词。注意，beginWord 不必是字典 wordList 中的单词。
# sk == endWord
# 给你两个单词 beginWord 和 endWord ，以及一个字典 wordList 。请你找出并返回所有从 beginWord 到 endWord 的 最短转换序列 ，
# 如果不存在这样的转换序列，返回一个空列表。每个序列都应该以单词列表 [beginWord, s1, s2, ..., sk] 的形式返回。


from collections import defaultdict, deque
from typing import List
from pprint import pprint


def findLadders(beginWord: str, endWord: str, wordList: List[str]) -> List[List[str]]:
    # 如果endWord不在wordList中，直接返回空列表
    if endWord not in wordList:
        return []

    # 获取beginWord的长度
    L = len(beginWord)

    # 使用defaultdict建立字典，字典的值是一个列表
    all_combo_dict = defaultdict(list)

    # 遍历wordList，建立通用状态到所有可达单词的映射
    for word in wordList:
        for i in range(L):
            all_combo_dict[word[:i] + "*" + word[i+1:]].append(word)

    # 创建一个双端队列，将beginWord和其对应路径作为起点
    queue = deque([(beginWord, [beginWord])])
    # 记录已经访问过的单词
    visited = {beginWord: True}
    # 最短路径结果
    ans = []

    # 进行广度优先搜索
    while queue and not ans:
        # 记录在这一层访问过的节点，防止在这一层出现重复访问
        level_visited = {}
        # 遍历当前层的所有节点
        for _ in range(len(queue)):
            # 取出队列的首部节点
            current_word, l = queue.popleft()
            # 遍历当前单词的所有字符
            for i in range(L):
                # 构建当前单词的通用状态
                intermediate_word = current_word[:i] + "*" + current_word[i+1:]
                # 遍历和当前通用状态相同的单词
                for word in all_combo_dict[intermediate_word]:
                    # 如果找到了endWord，将当前路径添加到结果列表
                    if word == endWord:
                        if ans is None:
                            ans = []
                        ans.append(l + [word])
                    # 如果该单词没有被访问过，标记为已访问并添加到队列中
                    if word not in visited:
                        level_visited[word] = True
                        queue.append((word, l + [word]))
                pprint(queue)
            # 更新已访问过的节点
            visited.update(level_visited)

    # 返回所有最短路径
    return ans


pprint(findLadders(beginWord="hit", endWord="cog",
       wordList=["hot", "dot", "dog", "lot", "log", "cog"]))
pprint(findLadders(beginWord="hot", endWord="dog", wordList=["hot", "dog"]))
# "hit" -> "hot" -> "dot" -> "dog" -> "cog"
# "hit" -> "hot" -> "lot" -> "log" -> "cog"
