# 470_用_Rand7()_实现_Rand10()
# https://leetcode.cn/problems/implement-rand10-using-rand7/description/

# The rand7() API is already defined for you.
# def rand7():
# @return a random integer in the range 1 to 7

class Solution:
    def rand10(self):
        def rand2():
            ans = rand7()
            if ans == 7:
                return rand2()
            else:
                return ans % 2
                
        ans = 1 * rand2() + 2 * rand2() + 4 * rand2() + 8 * rand2()
        if 1 <= ans <= 10:
            return ans
        else:
            return self.rand10()


# The rand7() API is already defined for you.
# def rand7():
# @return a random integer in the range 1 to 7
class Solution:
    def rand10(self):
        """
        :rtype: int
        """
        res = (rand7() -1) * 7 + rand7()
        if res > 40:
            return self.rand10()
        else:
            return 1 + (res - 1) % 10
            