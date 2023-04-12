class Solution:
    def longestDecomposition(self, text: str) -> int:
        i, j = 0, len(text)-1
        count = 0
        left = ''
        right = ''
        
        while i <= j:
            left += text[i]
            right = text[j] + right
            
            if i == j:
                break
            
            else:
                if left == right:
                    left = ''
                    right = ''
                    count += 2
                    
            i += 1
            j -= 1
            
        if left != '' and right != '':
            count += 1
            
        return count
