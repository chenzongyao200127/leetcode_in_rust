// 503_下一个更大元素_II
// https://leetcode.cn/problems/next-greater-element-ii/description/

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *nextGreaterElements(int *nums, int numsSize, int *returnSize)
{
    int *res = (int *)malloc(sizeof(int) * numsSize);
    *returnSize = numsSize;
    memset(res, -1, sizeof(int) * numsSize);
    int *stack = (int *)malloc(sizeof(int) * numsSize * 2);
    int top = -1;
    for (int i = 0; i < numsSize * 2; i++)
    {
        while (top != -1 && nums[stack[top]] < nums[i % numsSize])
        {
            res[stack[top]] = nums[i % numsSize];
            top--;
        }
        stack[++top] = i % numsSize;
    }
    free(stack);
    return res;
}