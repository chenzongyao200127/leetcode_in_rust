// 546_Remove_Boxes


// 1 <= boxes.length <= 100
// 1 <= boxes[i] <= 100

impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let mut dp = [[[0; 100]; 100]; 100];

        fn calculate_points(boxes: &[i32], l: usize, r: isize, k: usize, dp: &mut [[[i32; 100]; 100]; 100]) -> i32 {
            if l as isize > r {
                return 0;
            }

            if dp[l][r as usize][k] == 0 {
                dp[l][r as usize][k] = calculate_points(boxes, l, r - 1, 0, dp) + ((k + 1) * (k + 1)) as i32;

                for i in l..r as usize {
                    if boxes[i] == boxes[r as usize] {
                        dp[l][r as usize][k] = dp[l][r as usize][k].max(
                            calculate_points(boxes, l, i as isize, k+1, dp) + 
                            calculate_points(boxes, i+1, r - 1, 0, dp)
                        );
                    }
                }
            }

            dp[l][r as usize][k]
        }

        if boxes.is_empty() {
            return 0;
        }
        
        calculate_points(&boxes, 0, boxes.len() as isize - 1, 0, &mut dp);
        dp[0][(boxes.len() as isize - 1) as usize][0]
    }
}