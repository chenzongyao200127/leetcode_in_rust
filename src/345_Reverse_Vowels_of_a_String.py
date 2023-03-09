# 345_Reverse_Vowels_of_a_String
# https://leetcode.cn/problems/reverse-vowels-of-a-string/

class Solution:
    def reverseVowels(self, s: str) -> str:
        vowels = ['a','e','i','o','u','A','E','I','O','U']
        n = len(s)
        s = list(s)
        left = 0
        right = n-1
        while left < right:
            while left < right and not s[left] in vowels:
                left += 1
            while left < right and not s[right] in vowels:
                right -= 1
            s[left], s[right] = s[right], s[left]
            left += 1
            right -= 1
                
        return "".join(s)
    
    
# class Solution:
#     def reverseVowels(self, s: str) -> str:
#         def isVowel(ch: str) -> bool:
#             return ch in "aeiouAEIOU"
        
#         n = len(s)
#         s = list(s)
#         i, j = 0, n - 1
#         while i < j:
#             while i < n and not isVowel(s[i]):
#                 i += 1
#             while j > 0 and not isVowel(s[j]):
#                 j -= 1
#             if i < j:
#                 s[i], s[j] = s[j], s[i]
#                 i += 1
#                 j -= 1
        
#         return "".join(s)

# 作者：LeetCode-Solution
# 链接：https://leetcode.cn/problems/reverse-vowels-of-a-string/solution/fan-zhuan-zi-fu-chuan-zhong-de-yuan-yin-2bmos/
# 来源：力扣（LeetCode）
# 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。