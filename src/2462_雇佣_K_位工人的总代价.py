# 2462_雇佣_K_位工人的总代价
# https://leetcode.cn/problems/total-cost-to-hire-k-workers/description/?envType=daily-question&envId=2024-05-01

from typing import List
from heapq import heapify, heapreplace


class Solution:
    def totalCost(self, costs: List[int], k: int, candidates: int) -> int:
        n = len(costs)
        if candidates * 2 + k > n:
            # 也可以 sum(nsmallest(k, costs))，但效率不如直接排序
            costs.sort()
            return sum(costs[:k])

        pre = costs[:candidates]
        suf = costs[-candidates:]
        heapify(pre)
        heapify(suf)

        ans = 0
        i = candidates
        j = n - 1 - candidates
        for _ in range(k):
            if pre[0] <= suf[0]:
                ans += heapreplace(pre, costs[i])
                i += 1
            else:
                ans += heapreplace(suf, costs[j])
                j -= 1
        return ans
