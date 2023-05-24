class Solution:
    def bestTeamScore(self, scores: List[int], ages: List[int]) -> int:
        n = len(scores)
        if n == 1:
            return scores[0]
        mans = []
        for man in zip(ages,scores):
            mans.append(man)
        mans.sort()
        
        ans = [mans[x][1] for x in range(n)]
        for i in range(1,n):
            for j in range(i):
                if mans[j][1] <= mans[i][1]:
                    ans[i] = max(ans[i],mans[i][1] + ans[j])

        
        return max(ans)