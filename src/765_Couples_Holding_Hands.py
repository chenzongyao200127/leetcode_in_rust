# 765_Couples_Holding_Hands
# https://leetcode.cn/problems/couples-holding-hands/?envType=daily-question&envId=2023-11-11


from typing import List

class Solution:
    def minSwapsCouples(self, row: List[int]) -> int:
        # Create a map to store the index of each person in the row
        position = {person: i for i, person in enumerate(row)}
        swaps = 0
        
        for i in range(0, len(row), 2):
            # Find the correct partner for the current person
            x = row[i]
            y = x + 1 if x % 2 == 0 else x - 1
            
            # If the partner is not in the correct position, swap them
            if row[i + 1] != y:
                partner_idx = position[y]

                # Swap the partners
                row[i + 1], row[partner_idx] = row[partner_idx], row[i + 1]

                # Update their positions in the map
                position[row[partner_idx]] = partner_idx
                position[row[i + 1]] = i + 1

                swaps += 1
        
        return swaps

s = Solution()
ans = s.minSwapsCouples([0,2,1,3])
print(ans)
            
s = Solution()
ans = s.minSwapsCouples([3,2,0,1])
print(ans)

s = Solution()
ans = s.minSwapsCouples([5,4,2,6,3,1,0,7])
print(ans)
