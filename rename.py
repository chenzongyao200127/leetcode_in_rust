input_string = input("请输入一个字符串：")

# 用 replace() 函数替换空格和点
modified_string = input_string.replace(' ', '_').replace('.', '')

print("修改后的字符串是：", modified_string)

# from datetime import datetime, timedelta
# import re
# import os

# # 定义艾宾浩斯遗忘曲线的复习天数
# REVIEW_DAYS = [1, 2, 4, 7, 15]

# # 读取题号函数


# def read_problem_numbers(file_path):
#     problems = []
#     with open(file_path, 'r', encoding='utf-8') as file:
#         text = file.read()
#         # 使用正则表达式匹配所有题号
#         matches = re.findall(r'\b\d+\b', text)
#         problems.extend(matches)
#     return problems

# # 获取今天和之前的题目函数


# def get_problems_for_today(all_problems, start_date, today):
#     # 计算今天是学习计划的第几天
#     delta_days = (today - start_date).days
#     # 选择今天的新题
#     new_problems_index = delta_days * 3
#     new_problems = all_problems[new_problems_index: new_problems_index + 3]

#     # 选择旧题
#     old_problems = []
#     for review_day in REVIEW_DAYS:
#         review_date = today - timedelta(days=review_day)
#         review_day_index = (review_date - start_date).days * 3
#         if review_day_index >= 0:
#             old_problems.extend(
#                 all_problems[review_day_index: review_day_index + 3])

#     # 限制旧题数量到3道
#     old_problems = old_problems[-3:]

#     return new_problems, old_problems

# # 主程序


# def main():
#     problems_file_path = 'Order.txt'  # txt文件路径
#     start_date = datetime(2024, 4, 1)    # 学习计划开始日期
#     today = datetime.now()               # 今天的日期

#     # 检查文件是否存在
#     if not os.path.exists(problems_file_path):
#         print(f"The file {problems_file_path} does not exist.")
#         return

#     all_problems = read_problem_numbers(problems_file_path)
#     new_problems, old_problems = get_problems_for_today(
#         all_problems, start_date, today)

#     # 输出今天的学习题目
#     print("Today's new problems:", new_problems)
#     print("Today's review problems:", old_problems)


# if __name__ == '__main__':
#     main()
