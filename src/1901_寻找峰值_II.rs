// 1901_寻找峰值_II
// https://leetcode.cn/problems/find-a-peak-element-ii/description/

pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let rows = mat.len();
    let cols = mat[0].len();
    let mut l = 0;
    let mut r = cols - 1;

    while l <= r {
        let mid = l + (r - l) / 2;

        // Find the maximum element in the middle column
        let (mut idx, mut max_num_in_center) = (0, 0);
        for i in 0..rows {
            if mat[i][mid] > max_num_in_center {
                idx = i;
                max_num_in_center = mat[i][mid];
            }
        }

        // Check if the maximum element is a peak
        let left_is_smaller = mid == 0 || max_num_in_center > mat[idx][mid - 1];
        let right_is_smaller = mid == cols - 1 || max_num_in_center > mat[idx][mid + 1];

        if left_is_smaller && right_is_smaller {
            return vec![idx as i32, mid as i32];
        } else if !left_is_smaller {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    unreachable!()
}
