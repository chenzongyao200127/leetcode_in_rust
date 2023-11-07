# 2103_Rings_and_Rods
# https://leetcode.cn/problems/rings-and-rods/description/?envType=daily-question&envId=2023-11-02

class Solution:
    def countPoints(self, rings: str) -> int:
        colors = ['B', 'G', 'R']
        idx = 0
        cnts = [set() for _ in range(10)]
        while idx < len(rings):
            if rings[idx] in colors:
                j = idx+1
                while j < len(rings) and rings[j] not in colors:
                    j += 1
                # print(idx, j)
                if j < len(rings):
                    cnts[int(rings[idx+1:j])].add(rings[idx])
                    print(cnts)
                    idx = j
                else:
                    break
                
        
        ans = 0
        for n in cnts:
            if len(n) == 3:
                ans += 1
        return ans 
        

s = Solution()
ans = s.countPoints("B9R9G0R4G6R8R2R9G9")
print(ans)
                
                
class Solution:
    def countPoints(self, rings: str) -> int:
        # Use a list comprehension to create unique sets for each rod
        cnts = [set() for _ in range(10)]
        # Process the rings string two characters at a time
        for i in range(0, len(rings), 2):
            color = rings[i]
            rod = int(rings[i+1])
            cnts[rod].add(color)
        
        # Count the number of rods with all three colors
        return sum(len(rod_colors) == 3 for rod_colors in cnts)

# Testing the code
s = Solution()
ans = s.countPoints("B0B6G0R6R0R6G9")
print(ans)
                