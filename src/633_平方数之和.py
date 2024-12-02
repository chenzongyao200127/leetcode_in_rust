# 633_平方数之和

class Solution:
    def judgeSquareSum(self, c: int) -> bool:
        left, right = 0, int(c ** 0.5)
        while left <= right:
            cur = left ** 2 + right ** 2
            if cur == c:
                return True
            elif cur < c:
                left += 1
            else:
                right -= 1
        return False
