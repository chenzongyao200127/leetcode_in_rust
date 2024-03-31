# 快速幂算法（Fast Exponentiation Algorithm），也称为快速幂乘算法，
# 是一种用于快速计算 `a` 的 `n` 次幂 (`a^n`) 的有效方法，尤其是当 `n` 很大时。
# 该算法的基本思想是将指数 `n` 分解为2的幂的和，并利用幂的性质来减少乘法的次数。

# 以下是快速幂算法的 Python 实现，其中包括迭代和递归两种版本：

# 迭代版本


def fast_pow_iterative(base, exponent, modulus=None):
    result = 1
    base %= modulus if modulus else base

    while exponent > 0:
        # 如果当前指数为奇数，则将当前的base乘到结果中
        if exponent % 2 == 1:
            result = (result * base) % modulus if modulus else result * base
        # 指数除以2，base自乘
        exponent //= 2
        base = (base * base) % modulus if modulus else base * base

    return result


# 递归版本


def fast_pow_recursive(base, exponent, modulus=None):
    if exponent == 0:
        return 1
    if exponent % 2 == 0:
        half_pow = fast_pow_recursive(base, exponent // 2, modulus)
        return (half_pow * half_pow) % modulus if modulus else half_pow * half_pow
    else:
        half_pow = fast_pow_recursive(base, (exponent - 1) // 2, modulus)
        result = (base * half_pow *
                  half_pow) % modulus if modulus else base * half_pow * half_pow
        return result


# 在这两个版本中，可选参数 `modulus` 允许你计算模幂，即 `a^n (mod m)`。这在密码学和数论中特别有用。

# 快速幂算法的时间复杂度是 `O(log n)`，因为它将问题的规模每次至少减少一半。

# 使用例子：


print(fast_pow_iterative(2, 10))  # Output: 1024
print(fast_pow_recursive(2, 10))  # Output: 1024


# 如果你需要计算模幂，例如 `2^10 (mod 1000)`，你可以传入 `modulus` 参数：


print(fast_pow_iterative(2, 10, 1000))  # Output: 24
print(fast_pow_recursive(2, 10, 1000))  # Output: 24

# 请注意，在实际使用中，当涉及到大整数和模运算时，Python 的内置 `pow()` 函数已经非常高效，支持第三个参数作为模数：


print(pow(2, 10, 1000))  # Output: 24


# 这是因为 `pow()` 内部实现了快速幂算法，并且经过了优化。所以在没有特殊要求的情况下，推荐直接使用内置的 `pow()` 函数。


# 当然，快速幂算法的基本原理很简单，我们可以通过一个例子来详细了解它的过程。
# 假设我们想计算 `a^b` 的值。普通的方法是将 `a` 乘以自身 `b-1` 次，但这种方法的时间复杂度是 `O(b)`，效率不高。
# 快速幂算法通过二进制展开指数 `b` 来减少乘法的次数。

# ### 快速幂算法的步骤

# 1. **二进制展开：** 首先，将指数 `b` 展开成二进制表示方式。比如 `b = 13` 可以写成 `b = 1 * 2^3 + 1 * 2^2 + 0 * 2^1 + 1 * 2^0`。

# 2. **从左到右处理：** 然后，从最高位的二进制位开始，对于每一个二进制位 `1`，我们计算 `a` 的 `2^i` 次幂，其中 `i` 是该二进制位的权重。

# 3. **累乘所需的幂：** 当我们遍历到第 `i` 个位时（从0开始），如果这一位是 `1`，我们就将之前计算的结果（初始为1）乘以 `a` 的 `2^i` 次幂。

# 4. **平方运算：** 每向右移动一位（即每处理一个更低位的二进制位），我们不断地将 `a` 自身平方，因为二进制的每一位相当于权重翻倍。

# ### 例子

# 让我们计算 `3^13`。首先，13 的二进制表示是 `1101`。

# 1. 初始化结果 `res = 1` 和当前幂 `curr_power = a = 3`。
# 2. 从左到右遍历指数的二进制位：
#    - 第一位是 `1`，所以 `res = res * curr_power = 1 * 3 = 3`。
#    - `curr_power` 自平方得到 `3^2 = 9`。
#    - 第二位是 `1`，所以 `res = res * curr_power = 3 * 9 = 27`。
#    - `curr_power` 自平方得到 `9^2 = 81`。
#    - 第三位是 `0`，我们不更新 `res`，只更新 `curr_power`。
#    - `curr_power` 自平方得到 `81^2` (我们不需要精确值，因为它会变得很大)。
#    - 第四位是 `1`，所以 `res = res * curr_power = 27 * 81^2`（这是最终结果）。

# 这个过程只需进行 `log2(b)` 次乘法，相比于普通方法 `b-1` 次乘法，大幅度减少了计算量。

# ### 模幂运算

# 在许多应用中，特别是在密码学中，我们对 `a^b mod m` 更感兴趣。
# 快速幂算法可以很容易地集成模运算，因为 `(a * b) mod m = ((a mod m) * (b mod m)) mod m` 这个性质允许我们在每次乘法后立即取模，以保持整数大小的可管理性。

# 快速幂算法的优点在于它的效率和可扩展性。无论是在大数运算还是模幂运算中，它都是一个非常有用的工具。
# 然而，当涉及到非常大的数和模数时，应该使用专门的库来处理这些大数运算，以便利用更高级的算法和优化。在 Python 中，内置的 `pow()` 函数已经足够高效，因为它使用了类似的优化算法。
