import sys

# 假设第一行不是测试用例，而是其他信息，例如测试用例的数量，我们不需要它
group = int(sys.stdin.readline())

# print(group)
# 读取每个测试用例
# for line in sys.stdin:
for _ in range(group):
    # 将每行的字符串拆分为整数列表
    nums = list(map(int, sys.stdin.readline().strip().split(',')))
    # 进行一些处理，这里我们只是打印出来
    print(sum(nums))


line = sys.stdin.readline().strip()

for line in sys.stdin:
    nums = list(map(int, line.strip().split(' ')))
