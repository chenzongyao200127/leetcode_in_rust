input_string = input("请输入一个字符串：")

# 用 replace() 函数替换空格和点
modified_string = input_string.replace(' ', '_').replace('.', '')

print("修改后的字符串是：", modified_string)
