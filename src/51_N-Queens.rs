// 51. N-Queens
// https://leetcode.cn/problems/n-queens/

// Backtracking Algorithm
// 按照国际象棋的规则，皇后可以攻击与之处在同一行或同一列或同一斜线上的棋子。
// n 皇后问题 研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
// 给你一个整数 n ，返回所有不同的 n 皇后问题 的解决方案。
// 每一种解法包含一个不同的 n 皇后问题 的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。
// 输入：n = 4
// 输出：[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]impl Solution {
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {

        fn make_ans(pos: Vec<Vec<usize>>) -> Vec<Vec<String>> {
            let mut ans = vec![];
            for cols in pos.iter() {
                let len = cols.len();
                let mut solution = vec![];
                for &line in cols {
                    let mut s = ".".repeat(len).to_string();
                    s.replace_range(line..line + 1, "Q");
                    solution.push(s);
                }
                ans.push(solution);
            }
            ans
        }

        fn not_valid(cols: &Vec<usize>, cur: usize) -> bool {
            cols.iter().enumerate().any(|(r, &c)| {
                cur == c
                    || cols.len() == r
                    || (cur as i64 - c as i64).abs() == (cols.len() as i64 - r as i64).abs()
            })
        }

        fn dfs(n: usize, cols: &mut Vec<usize>, ans: &mut Vec<Vec<usize>>) {
            if cols.len() == n {
                ans.push(cols.to_vec());
                return;
            }
            for cur in 0..n {
                if not_valid(cols, cur) {
                    continue;
                }
                cols.push(cur);
                dfs(n, cols, ans);
                cols.pop();
            }
        }
        
        let mut ans = vec![];
        dfs(n as usize, &mut vec![], &mut ans);
        make_ans(ans)
    }
}




// class Solution {
//     private:
//     vector<vector<string>> result;
//     // n 为输入的棋盘大小
//     // row 是当前递归到棋盘的第几行了
//     void backtracking(int n, int row, vector<string>& chessboard) {
//         if (row == n) {
//             result.push_back(chessboard);
//             return;
//         }
//         for (int col = 0; col < n; col++) {
//             if (isValid(row, col, chessboard, n)) { // 验证合法就可以放
//                 chessboard[row][col] = 'Q'; // 放置皇后
//                 backtracking(n, row + 1, chessboard);
//                 chessboard[row][col] = '.'; // 回溯，撤销皇后
//             }
//         }
//     }
//     bool isValid(int row, int col, vector<string>& chessboard, int n) {
//         // 检查列
//         for (int i = 0; i < row; i++) { // 这是一个剪枝
//             if (chessboard[i][col] == 'Q') {
//                 return false;
//             }
//         }
//         // 检查 45度角是否有皇后
//         for (int i = row - 1, j = col - 1; i >=0 && j >= 0; i--, j--) {
//             if (chessboard[i][j] == 'Q') {
//                 return false;
//             }
//         }
//         // 检查 135度角是否有皇后
//         for(int i = row - 1, j = col + 1; i >= 0 && j < n; i--, j++) {
//             if (chessboard[i][j] == 'Q') {
//                 return false;
//             }
//         }
//         return true;
//     }
//     public:
//         vector<vector<string>> solveNQueens(int n) {
//             result.clear();
//             std::vector<std::string> chessboard(n, std::string(n, '.'));
//             backtracking(n, 0, chessboard);
//             return result;
//         }
//     };
    
//     作者：carlsun-2
//     链接：https://leetcode.cn/problems/n-queens/solution/dai-ma-sui-xiang-lu-51-n-queenshui-su-fa-2k32/
//     来源：力扣（LeetCode）
//     著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。