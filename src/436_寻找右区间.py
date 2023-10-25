# 436_寻找右区间
# https://leetcode.cn/problems/find-right-interval/description/

from typing import List

class Solution:
    def findRightInterval(self, intervals: List[List[int]]) -> List[int]:
        # map = {}
        # for idx, interval in enumerate(intervals):
        #     map[interval[0]] = idx
        
        map = {interval[0]: idx for idx, interval in enumerate(intervals)}
        
        
        # arr = []
        # for int in intervals:
        #     arr.append(int[0])
        # arr = sorted(arr)
        
        arr = [interval[0] for interval in intervals]
        arr = sorted(arr)
        
        
        def search(interval):
            target = interval[1]
        
        # def search(target):
            if target > arr[-1]:
                return -1
            
            l = 0
            r = len(arr)-1
            
            while l <= r:
                mid = l + (r - l) // 2
                print((l, r, mid))
                if arr[mid] < target:
                    l = mid + 1
                elif arr[mid] > target:
                    r = mid - 1
                else:
                    print(map[arr[r]])
                    return map[arr[r]]
            
            return map[arr[l]]
            
        ans = []
        for int in intervals:
            print(int[1])
            ans.append(search(int))
            print(ans)
        
        return ans

s = Solution()

ans = s.findRightInterval([[1,12],[2,9],[3,10],[13,14],[15,16],[16,17]])
print(ans)       
assert ans == [3,3,3,4,5,-1]           
            

from typing import List

class Solution:
    def findRightInterval(self, intervals: List[List[int]]) -> List[int]:
        # Create a map to track the original index of the intervals
        map = {interval[0]: idx for idx, interval in enumerate(intervals)}
        
        # Extract starting points and sort them
        starts = [interval[0] for interval in intervals]
        sorted_starts = sorted(starts)
        
        # def search(target):
        #     l = 0
        #     r = len(sorted_starts) - 1
        #     while l <= r:
        #         mid = l + (r - l) // 2
        #         print((l, r, mid))
        #         if sorted_starts[mid] < target:
        #             l = mid + 1 # [mid + 1, r]
        #         else:
        #             r = mid - 1 # [l, mid - 1]

        #     # If l is out of bounds, no interval found.
        #     if l == len(sorted_starts):
        #         return -1
        #     return map[sorted_starts[l]]
        
        def search(target):
            if target > sorted_starts[-1]:
                return -1
                        
            l = 0
            r = len(sorted_starts) - 1
            while l <= r:
                mid = l + (r - l) // 2
                if sorted_starts[mid] < target:
                    l = mid + 1
                elif sorted_starts[mid] > target:
                    r = mid - 1
                else: # sorted_starts[mid] == target
                    return map[sorted_starts[mid]]

            # If l is out of bounds, no interval found.
            # if l == len(sorted_starts):
            #     return -1
            return map[sorted_starts[l]]

            
        return [search(interval[1]) for interval in intervals]

s = Solution()

ans = s.findRightInterval([[1,12],[2,9],[3,10],[13,14],[15,16],[16,17]])
print(ans)
assert ans == [3,3,3,4,5,-1]
            
            
            
class Solution:
    def findRightInterval(self, intervals: List[List[int]]) -> List[int]:
        # 对区间开始节点按照升序做数组排列
        dic = {}
        start = []
        for i, t in enumerate(intervals):
            start.append(t[0])
            dic[t[0]] = i
        start.sort()
        ret = []
        for s, e in intervals:
            index = bisect.bisect_left(start, e)
            if index < len(start):
                ret.append(dic[start[index]])
            else:
                ret.append(-1)
        return ret            