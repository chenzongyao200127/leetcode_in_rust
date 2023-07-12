# 433_Minimum_Genetic_Mutation
# https://leetcode.cn/problems/minimum-genetic-mutation/description/

from collections import defaultdict, deque
from typing import List
from pprint import pprint

class Solution:
    def minMutation(self, startGene: str, endGene: str, bank: List[str]) -> int:
        if endGene not in bank:
            return -1
        
        L = len(startGene)
        
        all_combo_dict = defaultdict(list)
        
        for gene in bank:
            for i in range(L):
                all_combo_dict[gene[:i] + "*" + gene[i+1:]].append(gene)
                
        
        queue = deque([(startGene, 0)])
        visited = {startGene: True}
        
        
        while queue:
            curr_gene, change_time = queue.popleft()
            for i in range(L):
                tmp_gene = curr_gene[:i] + "*" + curr_gene[i+1:]
                for gene in all_combo_dict[tmp_gene]:
                    if gene == endGene:
                        return change_time + 1
                    if gene not in visited:
                        visited[gene] = True
                        queue.append((gene, change_time + 1))
            all_combo_dict[tmp_gene] = []
            
        return -1
        