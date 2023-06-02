# https://leetcode.cn/problems/maximum-tastiness-of-candy-basket/

class Solution:
    def maximumTastiness(self, price: List[int], k: int) -> int:
        price.sort()
        left, right = 0, price[-1] - price[0]
        while left < right:
            mid = (left + right + 1) // 2
            if self.check(price, k, mid):
                left = mid
            else:
                right = mid - 1
        return left

    def check(self, price: List[int], k: int, tastiness: int) -> bool:
        prev = -inf
        cnt = 0
        for p in price:
            if p - prev >= tastiness:
                cnt += 1
                prev = p
        return cnt >= k



class Solution:
    def maximumTastiness(self, price: List[int], k: int) -> int:
        price.sort()

        def check(limit: int) -> bool:
            x1 = price[0] + limit
            count = 1
            for p in price:
                if p >= x1:
                    count += 1
                    x1 = p + limit
            return count >= k

        left = 1
        right = (price[-1] - price[0]) // (k - 1)
        while left <= right:
            mid = (left + right) // 2
            if check(mid):
                left = mid + 1
            else:
                right = mid - 1

        return right
