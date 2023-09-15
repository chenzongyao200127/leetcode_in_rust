// 367_Valid_Perfect_Square
// https://leetcode.cn/problems/valid-perfect-square/description/

#include<stdio.h>
#ifndef __cplusplus
#include <stdbool.h>
#endif

bool isPerfectSquare(int num){
    long long l = 0;
    long long r = num / 2 + 1;
    long long mid = 0;
    while (l <= r) {
        mid = l + (r - l) / 2;

        long long tmp = mid * mid;

        if (tmp < num) {
            l = mid + 1;
        }

        if (tmp > num) {
            r = mid - 1;
        }

        if (tmp == num) {
            return true;
        }
    }
    
    return false;
}