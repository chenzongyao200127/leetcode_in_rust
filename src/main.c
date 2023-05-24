#include <stdio.h> 
#include <stdlib.h>

#define MAX_NM 10
#define MAX_POSTAGE 1024
#define INF 2147483647
 
int n, m;
int x[MAX_NM], ans[MAX_NM], y[MAX_POSTAGE],  maxStamp, r;
 
/*
 * backtrack(i)表示x[0...i-1]这i张邮票已经完全确定，
 * 相应于x[0...i-1]的最大连续邮资区间r和每种邮资所需要的
 * 最少邮票张数y[0...r]也都确定，现在枚举x[i]
 * 的每个值，确定x[i]
 */
void backtrack(int i) {
    int *backup_y, backup_r;
    int next, postage, num, tmp;
 
    if(i >= n) {
        if(r > maxStamp) {
            maxStamp = r;
            for(tmp = 0; tmp < n; tmp++)
                ans[tmp] = x[tmp];
        }
        return;
    }

    backup_y = (int*)malloc(MAX_POSTAGE * sizeof(int));
    for(tmp = 0; tmp < MAX_POSTAGE; tmp++) backup_y[tmp] = y[tmp];
    backup_r = r;
 
    for(next = x[i - 1] + 1; next <= r + 1; next++) {
        /* update x[i] */
        x[i] = next;
        /* update y */
        for(postage = 0; postage < x[i-1] * m; postage++) {
            if(y[postage] >= m) continue;
            for(num = 1; num <= m - y[postage]; num++)
                if(y[postage] + num < y[postage + num * next]
                   && (postage + num * next < MAX_POSTAGE))
                    y[postage + num * next] = y[postage] + num;
        }
        /* update r */
        while(y[r + 1] < INF) r++;
 
        backtrack(i + 1);
 
        /* restore */
        r = backup_r;
        for(tmp = 0; tmp < MAX_POSTAGE; tmp++) y[tmp] = backup_y[tmp];
    }
    free(backup_y);
}

int main() {
    int i;
 
    // scanf("%d%d", &n, &m);
    n = 5;
    m = 4;

    x[0] = 1;
    r = m;
    for(i = 0; i <= r; i++) y[i] = i;
    while(i < MAX_POSTAGE) y[i++] = INF;
    maxStamp = 0;

    backtrack(1);
 
    printf("max stamp is: %d/n", maxStamp);
    for(i = 0; i < n; i++) printf("%4d", ans[i]);

    return 0;
}