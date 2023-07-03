# 2761_Prime_Pairs_With_Target_Sum
# https://leetcode.cn/problems/prime-pairs-with-target-sum/

# 给你一个整数 n 。如果两个整数 x 和 y 满足下述条件，则认为二者形成一个质数对：

# 1 <= x <= y <= n
# x + y == n
# x 和 y 都是质数
# 请你以二维有序列表的形式返回符合题目要求的所有 [xi, yi] ，列表需要按 xi 的 非递减顺序 排序。如果不存在符合要求的质数对，则返回一个空数组。

# 注意：质数是大于 1 的自然数，并且只有两个因子，即它本身和 1 。

from typing import List

# 厄拉多塞筛法是一种用来找出一定范围内所有素数的经典算法，命名来源于古希腊数学家厄拉多塞。
# 其主要思想是从最小的素数开始，每次标记该素数的所有倍数，没有被标记过的数就是新的素数。
# 以下是厄拉多塞筛法的一般步骤：

# 首先，列出从2开始的所有自然数，构造一个序列：2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20...

# 从序列的第一个数字（这里是2）开始，标记序列中该数字的所有倍数（除了该数字本身）。也就是说，你需要标记4, 6, 8, 10, 12...

# 你会发现，在这个步骤后，序列中的第二个数是3，它没有被标记，因为它是素数。
# 然后，你需要标记序列中3的所有倍数。所以，你会标记6, 9, 12, 15...

# 继续找下一个没有被标记的数字，然后标记它的所有倍数。这个数字是5，它的倍数包括10, 15, 20...

# 这个过程会继续进行，直到所有小于或等于你想要搜索的范围的数字都被处理。

# 经过这个过程，没有被标记的数字就是素数，因为它们没有任何其他的因数。

# 举个例子，我们要找出小于或等于20的所有素数。经过厄拉多塞筛法，我们将得到以下序列：

# 2, 3, 5, 7, 11, 13, 17, 19

# 这就是小于或等于20的所有素数。

# 需要注意的是，虽然厄拉多塞筛法在理论上非常有效，但在实际应用中，由于需要处理大量的数据，可能会面临内存问题。
# 为了解决这个问题，有一些改进的筛法被提出，例如线性筛、欧拉筛等。


# eratosthenes_sieve 函数实现了厄拉多塞筛法，用于找到所有小于等于 n 的质数。
# prime_pairs_optimized 函数使用了预先计算出的质数集合来查找符合题目要求的质数对。
# 这样，我们避免了在寻找质数对时重复检查每个数是否为质数，从而大大减少了运行时间。
class Solution:
    def findPrimePairs(self, n: int) -> List[List[int]]:
        def eratosthenes_sieve(n):
            is_prime = [True] * (n + 1)
            is_prime[0] = False
            is_prime[1] = False
            prime_list = []

            for i in range(2, int(n**0.5) + 1):
                if is_prime[i]:
                    prime_list.append(i)
                    for multiple in range(i * i, n + 1, i):
                        is_prime[multiple] = False
                        
            for i in range(int(n**0.5) + 1, n + 1):
                if is_prime[i]:
                    prime_list.append(i)

            return prime_list

        primes = set(eratosthenes_sieve(n))
        result = []

        for p in list(primes):
            if n - p in primes and p <= (n // 2) :
                result.append([p, n - p])
        result.sort()
        return result
    
    
# 线性筛法（Linear Sieve）：线性筛法是一种时间复杂度为O(n)的筛法，适用于筛选较大范围内的素数。
# 其基本原理和厄拉多塞筛法类似，但线性筛法在标记合数时有所不同：
# 当一个数被筛选到时，线性筛会让这个数只被它的最小质因数筛掉，避免了重复标记，从而达到了线性的时间复杂度。

def linear_sieve(n):
    prime = [0] * (n + 1)
    result = []
    for i in range(2, n + 1):
        if prime[i] == 0:
            result.append(i)
        j = 0
        while result[j] * i <= n:
            prime[result[j] * i] = 1
            if i % result[j] == 0:
                break
            j += 1
    return result


# 欧拉筛法（Euler Sieve）：欧拉筛法是一种基于欧拉函数的筛法，它的基本思路和线性筛法相同，
# 也是让每个合数只被它的最小质因数筛掉。但是欧拉筛法在实现上会更加简洁，性能也会更好一些。
def euler_sieve(n):
    prime = [0] * (n + 1)
    result = []
    for i in range(2, n + 1):
        if prime[i] == 0:
            result.append(i)
        for j in result:
            if i * j > n:
                break
            prime[i * j] = 1
            if i % j == 0:
                break
    return result

# 以上两种筛法都能在相对较短的时间内筛选出较大范围内的素数，比原始的厄拉多塞筛法更高效。
# 但是无论使用哪种筛法，都要注意在筛选非常大的数时可能会遇到内存问题。