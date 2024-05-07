// 1463_摘樱桃_II
// https://leetcode.cn/problems/cherry-pickup-ii/description/?envType=daily-question&envId=2024-05-07

#include <vector>
#include <algorithm>
#include <functional>

using namespace std;

class Solution
{
public:
    int cherryPickup(vector<vector<int>> &grid)
    {
        int numRows = grid.size();
        int numCols = grid[0].size();

        // Memoization table initialized to -1, indicating uncomputed states
        vector<vector<vector<int>>> dp(numRows, vector<vector<int>>(numCols, vector<int>(numCols, -1)));

        // Define a recursive lambda function for depth-first search with memoization
        function<int(int, int, int)> dfs = [&](int row, int col1, int col2) -> int
        {
            // Base case: check for out of bounds
            if (row == numRows || col1 < 0 || col1 >= numCols || col2 < 0 || col2 >= numCols)
            {
                return 0;
            }

            // Reference to the current dp state to avoid recomputation
            int &result = dp[row][col1][col2];
            if (result != -1)
            {
                return result;
            }

            // Compute the maximum cherries collectible by exploring all valid moves
            result = 0; // Initialize result for current state
            for (int newCol1 = col1 - 1; newCol1 <= col1 + 1; ++newCol1)
            {
                for (int newCol2 = col2 - 1; newCol2 <= col2 + 1; ++newCol2)
                {
                    result = max(result, dfs(row + 1, newCol1, newCol2));
                }
            }

            // Collect cherries from current row; avoid double counting if both robots meet
            result += grid[row][col1];
            if (col1 != col2)
            {
                result += grid[row][col2];
            }

            return result;
        };

        // Start the dfs from the top row, with robots starting from the leftmost and rightmost columns
        return dfs(0, 0, numCols - 1);
    }
};