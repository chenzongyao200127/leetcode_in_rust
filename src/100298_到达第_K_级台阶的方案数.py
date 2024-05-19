# 100298_到达第_K_级台阶的方案数
# https://leetcode.cn/contest/weekly-contest-398/problems/find-number-of-ways-to-reach-the-k-th-stair/


def is_special_array(nums, queries):
    n = len(nums)

    diff_parity = [0] * (n - 1)
    for i in range(n - 1):
        if (nums[i] % 2) != (nums[i + 1] % 2):
            diff_parity[i] = 1

    prefix_sum = [0] * n
    for i in range(1, n):
        prefix_sum[i] = prefix_sum[i - 1] + diff_parity[i - 1]

    answer = []
    for fromi, toi in queries:
        if toi == fromi:
            answer.append(True)
        else:
            total_diff = prefix_sum[toi] - prefix_sum[fromi]
            if total_diff == (toi - fromi):
                answer.append(True)
            else:
                answer.append(False)

    return answer


def digit_difference_sum(nums):
    n = len(nums)
    if n < 2:
        return 0

    L = len(str(nums[0]))

    frequency = [[0] * 10 for _ in range(L)]

    for num in nums:
        num_str = str(num)
        for i in range(L):
            digit = int(num_str[i])
            frequency[i][digit] += 1

    result = 0
    for i in range(L):
        for d in range(10):
            count_d = frequency[i][d]
            result += count_d * (n - count_d)

    result //= 2

    return result


# 100298. 到达第 K 级台阶的方案数
# https://leetcode-cn.com/contest/weekly-contest-398/problems/find-number-of-ways-to-reach-the-k-th-stair/
