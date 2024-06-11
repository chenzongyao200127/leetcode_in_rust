// 881_救生艇
// https://leetcode.cn/problems/boats-to-save-people/description/?envType=daily-question&envId=2024-06-10

class Solution {
    public int numRescueBoats(int[] people, int limit) {
        Arrays.sort(people);
        int left = 0, right = people.length - 1;
        int ans = 0;
        while (left <= right) {
            if (people[left] + people[right] <= limit) {
                left++;
            }
            right--;
            ans++;
        }
        return ans;
    }
}