input_string = input("请输入一个字符串:").strip()

# Replace spaces with underscores and remove dots
modified_string = input_string.replace(' ', '_').replace('.', '')

print("修改后的字符串是:" + modified_string)
