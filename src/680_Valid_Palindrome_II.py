# 680_Valid_Palindrome_II
# https://leetcode.cn/problems/valid-palindrome-ii/

class Solution:
    def validPalindrome(self, s: str) -> bool:
        # def validPalindrome1(self, s: str) -> bool:
        #     n = len(s)
        #     left = 0
        #     right = n-1
        #     s = list(s)
        #     while left < right:
        #         if s[left] != s[right]:
        #             return False
        #         left += 1
        #         right -= 1
        #     return True
        def checkPalindrome(low, high):
            i, j = low, high
            while i < j:
                if s[i] != s[j]:
                    return False
                i += 1
                j -= 1
            return True
        
        n = len(s)
        left = 0
        right = n-1
        while left < right:
            if s[left] != s[right]:
                return checkPalindrome(left, right-1) or checkPalindrome(left+1, right)
            left += 1
            right -= 1
            
        return True
            
class Solution:
    def validPalindrome(self, s: str) -> bool:
        def checkPalindrome(low, high):
            i, j = low, high
            while i < j:
                if s[i] != s[j]:
                    return False
                i += 1
                j -= 1
            return True

        low, high = 0, len(s) - 1
        while low < high:
            if s[low] == s[high]: 
                low += 1
                high -= 1
            else:
                return checkPalindrome(low + 1, high) or checkPalindrome(low, high - 1)
        return True
# 作者：LeetCode-Solution
# 链接：https://leetcode.cn/problems/valid-palindrome-ii/solution/yan-zheng-hui-wen-zi-fu-chuan-ii-by-leetcode-solut/
# 来源：力扣（LeetCode）
# 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。