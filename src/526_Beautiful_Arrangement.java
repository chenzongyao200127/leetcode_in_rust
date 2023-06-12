// https://leetcode.cn/problems/beautiful-arrangement/solution/gong-shui-san-xie-xiang-jie-liang-chong-vgsia/

class Solution {
    public int countArrangement(int n) {
        return dfs(n, 1, new boolean[n + 1]);
    }

    private int dfs(int n, int i, boolean[] visited) {
        if (i > n) {
            return 1;
        }

        int ans = 0;
        for (int num = 1; num <= n; num++) {
            if (!visited[num] && (num % i == 0 || i % num == 0)) {
                visited[num] = true;
                ans += dfs(n, i + 1, visited);
                visited[num] = false;
            }
        }

        return ans;
    }
}