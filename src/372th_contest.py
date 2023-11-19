# 371th_contest

class Solution:
    def findMinimumOperations(self, s1: str, s2: str, s3: str) -> int:
        if s1[0] != s2[0] or s1[0] != s3[0]:
            return -1

        operations = 0
        while True:
            if s1 == s2 == s3:
                return operations

            if len(s1) < 2 and len(s2) < 2 and len(s3) < 2:
                return -1

            max_len = max(len(s1), len(s2), len(s3))
            if len(s1) == max_len:
                s1 = s1[:-1]
            elif len(s2) == max_len:
                s2 = s2[:-1]
            else:
                s3 = s3[:-1]

            operations += 1


class Solution:
    def minimumSteps(self, s: str) -> int:
        
        totalBlacks = s.count('1')  
        currentBlacks = 0
        steps = 0

        for i in range(len(s)):
            if s[i] == '1':  
                finalPosition = len(s) - totalBlacks + currentBlacks
                steps += finalPosition - i
                currentBlacks += 1

        return steps



class Solution:
    def maximumXorProduct(self, a: int, b: int, n: int) -> int:
        MOD = 10**9 + 7
        x = 0

        for i in range(n - 1, -1, -1):
            bit_a = (a >> i) & 1
            bit_b = (b >> i) & 1

            if bit_a == bit_b:
                x |= (1 - bit_a) << i
            else:
                a_xor_x = a ^ x
                b_xor_x = b ^ x
                if a_xor_x < b_xor_x:
                    x |= (1 - bit_a) << i
                else:
                    x |= (1 - bit_b) << i

        return ((a ^ x) * (b ^ x)) % MOD


# # Example usage
# s = Solution()
# ans = s.maximumXorProduct(12,5,4) # 98
# print(ans)

# # Example usage
# s = Solution()
# ans = s.maximumXorProduct(6,7,5) # 930
# print(ans)

# # Example usage
# s = Solution()
# ans = s.maximumXorProduct(1,6,3) # 12
# print(ans)

# s = Solution()
# ans = s.maximumXorProduct(53449611838892,712958946092406,6) # 231850918
# print(ans) 
