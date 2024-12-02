#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int totalNQueens(int n)
{
    int count = 0;
    int *cols = (int *)malloc(n * sizeof(int));
    int *diag1 = (int *)malloc(2 * n * sizeof(int));
    int *diag2 = (int *)malloc(2 * n * sizeof(int));
    memset(cols, 0, n * sizeof(int));
    memset(diag1, 0, 2 * n * sizeof(int));
    memset(diag2, 0, 2 * n * sizeof(int));

    void dfs(int row)
    {
        if (row == n)
        {
            count++;
            return;
        }
        for (int col = 0; col < n; col++)
        {
            if (cols[col] || diag1[row + col] || diag2[row - col + n])
            {
                continue;
            }
            cols[col] = diag1[row + col] = diag2[row - col + n] = 1;
            dfs(row + 1);
            cols[col] = diag1[row + col] = diag2[row - col + n] = 0;
        }
    }

    dfs(0);
    free(cols);
    free(diag1);
    free(diag2);
    return count;
}