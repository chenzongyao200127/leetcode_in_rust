# // 48. Rotate Image
# // https://leetcode.cn/problems/rotate-image/

# // 48. 旋转图像
# // 给定一个 n × n 的二维矩阵 matrix 表示一个图像。请你将图像顺时针旋转 90 度。
# // 你必须在 原地 旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要 使用另一个矩阵来旋转图像。


class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:
        n=len(matrix)
        for i in range(n):      #i是行数
            for j in range(i):  #j是列数
                matrix[i][j],matrix[j][i]=matrix[j][i],matrix[i][j]   #实现对角线对称
        for m in range(n):
            matrix[m]=matrix[m][::-1]         #每一行逆序