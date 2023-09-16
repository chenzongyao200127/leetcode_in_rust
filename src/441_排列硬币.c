// 441_排列硬币
// https://leetcode.cn/problems/arranging-coins/description/


#include<stdio.h>
#ifndef __cplusplus
#include <stdbool.h>
#endif


int arrangeCoins(int n){
    long long l = 0;
    long long r = n; // Include potential square root in search space
    long long mid;
    long long x = n;

    
    if (n == 1){
        return 1;
    }

    while (l < r) {
        mid = l + (r - l) / 2;
        long long tmp = mid * (mid + 1);

        if (tmp < 2 * x) {
            l = mid + 1; 
        } else if (tmp > 2 * x) {
            r = mid; 
        } else {
            return mid; 
        }
    }

    return l - 1;
}


int arrangeCoins(int n){
    int left=1,right=n;
    while(left<right){
        int mid=(right-left+1)/2+left;
        if((long long)mid*(mid+1)<=(long long)2*n){
            left=mid;
        }
        else{
            right=mid-1;
        }       
    }
    return left;
}
