# 187_Repeated_DNA_Sequences
# https://leetcode.cn/problems/repeated-dna-sequences/description/?envType=daily-question&envId=2023-11-05

from typing import List
class Solution:
    def findRepeatedDnaSequences(self, s: str) -> List[str]:
        if len(s) <= 10:
            return []
        l = 0
        r = 10
        st = set()
        ans = []
        while r <= len(s):
            if s[l:r] in st:
                ans.append(s[l:r])
            else:
                st.add(s[l:r])
            l += 1
            r += 1
        
        return list(set(ans))
        
s = Solution()
ans = s.findRepeatedDnaSequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT")
print(ans)


from collections import defaultdict
from typing import List

class Solution:
    def findRepeatedDnaSequences(self, s: str) -> List[str]:
        # DNA sequence length
        L = 10
        
        # Mapping each nucleotide to a 2-bit binary representation
        bin_map = {
            'A': 0,  # 00
            'C': 1,  # 01
            'G': 2,  # 10
            'T': 3   # 11
        }
        
        n = len(s)
        
        # If the sequence is shorter than 10, return an empty list
        if n <= L:
            return []
        
        result = []
        rolling_hash = 0
        
        # Initialize the first L-1 nucleotides' rolling hash
        for ch in s[:L - 1]:
            rolling_hash = (rolling_hash << 2) | bin_map[ch]
        
        sequences_count = defaultdict(int)
        
        for i in range(n - L + 1):
            # Slide the window to consider the next nucleotide for hashing
            rolling_hash = ((rolling_hash << 2) | bin_map[s[i + L - 1]]) & ((1 << (L * 2)) - 1)
            
            # Increase the count of this hash
            sequences_count[rolling_hash] += 1
            
            # If the sequence appears twice, add it to the result
            if sequences_count[rolling_hash] == 2:
                result.append(s[i: i + L])
        
        return result
