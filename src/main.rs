


fn max_values(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut answer = Vec::new();

    for query in queries {
        let (xi, yi) = (query[0], query[1]);
        let mut max_value = -1;

        for j in 0..nums1.len() {
            if nums1[j] >= xi && nums2[j] >= yi {
                max_value = max_value.max(nums1[j] + nums2[j]);
            }
        }

        answer.push(max_value);
    }

    answer
}

fn main() {
    let nums1_1 = vec![4, 3, 1, 2];
    let nums2_1 = vec![2, 4, 9, 5];
    let queries1 = vec![vec![4, 1], vec![1, 3], vec![2, 5]];

    let nums1_2 = vec![3, 2, 5];
    let nums2_2 = vec![2, 3, 4];
    let queries2 = vec![vec![4, 4], vec![3, 2], vec![1, 1]];

    let nums1_3 = vec![2, 1];
    let nums2_3 = vec![2, 3];
    let queries3 = vec![vec![3, 3]];

    println!("{:?}", max_values(nums1_1, nums2_1, queries1)); // 输出：[6, 10, 7]
    println!("{:?}", max_values(nums1_2, nums2_2, queries2)); // 输出：[9, 9, 9]
    println!("{:?}", max_values(nums1_3, nums2_3, queries3)); // 输出：[-1]
}
