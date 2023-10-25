# 2698_求一个整数的惩罚数
# https://leetcode.cn/problems/find-the-punishment-number-of-an-integer/?envType=daily-question&envId=2023-10-25

class Solution:
    def punishmentNumber(self, n: int) -> int:
        nums = []
        
        def isPunishmentNum(x):
            s = str(x * x)

            def dfs(s, target):                
                for i in range(len(s)):
                    tmp = int(s[:i+1])
                    if tmp > target:
                        break
                    elif tmp == target and i == len(s) - 1 :
                        return True
                    elif dfs(s[i+1:], target-tmp):
                        return True

                return False

            return dfs(s, x)

        
        for i in range(1, n+1):
            if isPunishmentNum(i):
                nums.append(i)
        
        print(nums)
        ans = 0
        for n in nums:
            ans += n * n
        return ans
        
        
s = Solution()
ans = s.punishmentNumber(37)
print(ans)



def punishment_sum(n: int) -> int:
    """
    Calculate the sum of the squares of numbers up to n,
    where the square of the number can be split into digits 
    that sum up to the original number.
    """
    def can_split_to_sum(s: str, target: int) -> bool:
        """
        Check if the string s can be split into digits such that 
        they sum up to target.
        """
        for i in range(len(s)):
            tmp = int(s[:i + 1])
            if tmp > target:
                break
            if tmp == target and i == len(s) - 1:
                return True
            if can_split_to_sum(s[i + 1:], target - tmp):
                return True
        return False

    total = 0
    for i in range(1, n + 1):
        if can_split_to_sum(str(i * i), i):
            total += i * i

    return total
