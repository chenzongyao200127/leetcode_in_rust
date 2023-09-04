# 233_Number_of_Digit_One
# https://leetcode.cn/problems/number-of-digit-one/description/

# 给定一个整数 n，计算所有小于等于 n 的非负整数中数字 1 出现的个数。

# 贡献法
def countDigitOne(n: int) -> int:
    # mulk = 10^k, 用于定位当前考虑的位数（个位、十位、百位...）
    k, mulk = 0, 1
    total_ones = 0

    while n >= mulk:
        # 高位数字
        high_digits = n // (mulk * 10)
        # 当前位的数值
        current_digit = (n // mulk) % 10
        # 低位数字
        low_digits = n % mulk

        # 高位贡献：对于完整的10^k范围（例如00~99，100~199），当前位上1出现的次数为10^(k-1)
        total_ones += high_digits * mulk

        # 低位贡献：取决于当前位的数值
        if current_digit == 1:
            total_ones += low_digits + 1
        elif current_digit > 1: 
            total_ones += mulk

        k += 1
        mulk *= 10

    return total_ones

print(countDigitOne(2104))



class Solution:
    def countDigitOne(self, n: int) -> int:
        # base cases
        if n <= 0:
            return 0
        if n < 10:
            return 1

        length = len(str(n))
        power = 10 ** (length - 1)
        high = n // power
        last = n % power

        # count 1's in the highest place
        if high == 1:
            countHigh = last + 1
        else:
            countHigh = power

        # count 1's in the lower places
        countLower = high * (length - 1) * (power // 10)
        
        return countHigh + countLower + self.countDigitOne(last)
    
    
    
# 数位DP
