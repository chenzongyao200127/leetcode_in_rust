// 2813_子序列最大优雅度
// https://leetcode.cn/problems/maximum-elegance-of-a-k-length-subsequence/description/

import java.util.*;

class Solution {
    public long findMaximumElegance(int[][] items, int k) {
        // Sort the items in descending order based on the first element
        Arrays.sort(items, (a, b) -> b[0] - a[0]);
        int n = items.length;
        long tot = 0; // Variable to store the total elegance
        Set<Integer> vis = new HashSet<>(); // Set to keep track of visited elements
        Deque<Integer> dup = new ArrayDeque<>(); // Deque to store duplicate elements
        for (int i = 0; i < k; ++i) {
            int p = items[i][0], c = items[i][1];
            tot += p; // Add the elegance of the current item to the total
            if (!vis.add(c)) {
                dup.push(p); // If the element is already visited, push it to the deque
            }
        }
        long ans = tot + (long) vis.size() * vis.size(); // Calculate the initial answer
        for (int i = k; i < n; ++i) {
            int p = items[i][0], c = items[i][1];
            if (vis.contains(c) || dup.isEmpty()) {
                continue; // Skip the current item if it is already visited or deque is empty
            }
            vis.add(c); // Mark the current item as visited
            tot += p - dup.pop(); // Update the total by subtracting the popped element from the deque and adding
                                  // the elegance of the current item
            ans = Math.max(ans, tot + (long) vis.size() * vis.size()); // Update the answer if necessary
        }
        return ans; // Return the maximum elegance
    }
}