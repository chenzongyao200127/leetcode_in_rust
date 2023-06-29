
pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; colsum.len()]; 2];
    let mut upper_remaining = upper;
    let mut lower_remaining = lower;

    for i in 0..colsum.len() {
        if colsum[i] == 2 {
            matrix[0][i] = 1;
            matrix[1][i] = 1;
            upper_remaining -= 1;
            lower_remaining -= 1;
        }
    }

    for i in 0..colsum.len() {
        if colsum[i] == 1 {
            if upper_remaining > 0 {
                matrix[0][i] = 1;
                upper_remaining -= 1;
            } else {
                matrix[1][i] = 1;
                lower_remaining -= 1;
            }
        }
    }

    if upper_remaining == 0 && lower_remaining == 0 {
        matrix
    } else {
        vec![]
    }
}

// pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
//     let mut matrix = vec![vec![0; colsum.len()]; 2];
//     for i in 0..colsum.len() {
//         if colsum[i] == 2 {
//             matrix[0][i] = 1;
//             matrix[1][i] = 1;    
//         }
//     }

//     dfs(0, upper, lower, &colsum, &mut matrix);
//     return matrix
// }

// pub fn dfs(cur_col: usize, upper: i32, lower: i32, colsum: &Vec<i32>, matrix: &mut Vec<Vec<i32>>) {
//     if cur_col == colsum.len() {
//         if colsum[cur_col] == 1 {
//             matrix[0][cur_col] = 1;
//             if is_valid(upper, lower, matrix) {
//                 return
//             }
//             matrix[0][cur_col] = 0;
//             matrix[1][cur_col] = 0;
//             if is_valid(upper, lower, matrix) {
//                 return
//             }
//         } else {
//             if is_valid(upper, lower, matrix) {
//                 return
//             }
//         }
//     }
//     for i in 0..=1 {
//         matrix[0][cur_col] = 1;
//         let mut next_col = i+1;
//         while next_col < colsum.len() && colsum[next_col] != 1 {
//             next_col += 1;
//         }
//         dfs(next_col, upper, lower, colsum, matrix);
//         matrix[0][cur_col] = 0;
//     }
// }

// pub fn is_valid(upper: i32, lower: i32, matrix: &Vec<Vec<i32>>) -> bool {
//     if matrix[0].iter().sum() == upper && matrix[1].iter().sum() == lower {
//         return true;
//     }
//     false
// }


fn main() {
    println!("{:?}", reconstruct_matrix(2,1,vec![1,1,1]))
}