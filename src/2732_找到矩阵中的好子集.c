// 2732_找到矩阵中的好子集
// https://leetcode.cn/problems/find-a-good-subset-of-the-matrix/description/
#include <stdio.h>;
#include <stdlib.h>;
#include <stdbool.h>;
#include <string.h>;
#include <math.h>;

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
typedef struct
{
    int key;
    int val;
    UT_hash_handle hh;
} HashItem;

HashItem *hashFindItem(HashItem **obj, int key)
{
    HashItem *pEntry = NULL;
    HASH_FIND_INT(*obj, &key, pEntry);
    return pEntry;
}

bool hashAddItem(HashItem **obj, int key, int val)
{
    if (hashFindItem(obj, key))
    {
        return false;
    }
    HashItem *pEntry = (HashItem *)malloc(sizeof(HashItem));
    pEntry->key = key;
    pEntry->val = val;
    HASH_ADD_INT(*obj, key, pEntry);
    return true;
}

bool hashSetItem(HashItem **obj, int key, int val)
{
    HashItem *pEntry = hashFindItem(obj, key);
    if (!pEntry)
    {
        hashAddItem(obj, key, val);
    }
    else
    {
        pEntry->val = val;
    }
    return true;
}

int hashGetItem(HashItem **obj, int key, int defaultVal)
{
    HashItem *pEntry = hashFindItem(obj, key);
    if (!pEntry)
    {
        return defaultVal;
    }
    return pEntry->val;
}

void hashFree(HashItem **obj)
{
    HashItem *curr = NULL, *tmp = NULL;
    HASH_ITER(hh, *obj, curr, tmp)
    {
        HASH_DEL(*obj, curr);
        free(curr);
    }
}

int *goodSubsetofBinaryMatrix(int **grid, int gridSize, int *gridColSize, int *returnSize)
{
    int *ans = NULL;
    HashItem *mp = NULL;
    int m = gridSize;
    int n = gridColSize[0];

    for (int j = 0; j < m; j++)
    {
        int st = 0;
        for (int i = 0; i < n; i++)
        {
            st |= (grid[j][i] << i);
        }
        hashAddItem(&mp, st, j);
    }
    if (hashFindItem(&mp, 0))
    {
        ans = (int *)malloc(sizeof(int));
        ans[0] = hashGetItem(&mp, 0, 0);
        *returnSize = 1;
        hashFree(&mp);
        return ans;
    }

    for (HashItem *pEntry1 = mp; pEntry1 != NULL; pEntry1 = pEntry1->hh.next)
    {
        int x = pEntry1->key, i = pEntry1->val;
        for (HashItem *pEntry2 = mp; pEntry2 != NULL; pEntry2 = pEntry2->hh.next)
        {
            int y = pEntry2->key, j = pEntry2->val;
            if (!(x & y))
            {
                ans = (int *)malloc(sizeof(int) * 2);
                ans[0] = fmin(i, j);
                ans[1] = fmax(i, j);
                *returnSize = 2;
                hashFree(&mp);
                return ans;
            }
        }
    }
    *returnSize = 0;
    hashFree(&mp);
    return ans;
}