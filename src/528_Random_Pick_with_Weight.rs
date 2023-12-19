// 528_Random_Pick_with_Weight
// https://leetcode.cn/problems/random-pick-with-weight/description/

use rand::Rng; // For random number generation
use std::iter::Iterator; // For iterator methods

struct Solution {
    cumulative_weights: Vec<f64>,
}

impl Solution {
    fn new(weights: Vec<i32>) -> Self {
        let total_weight: f64 = weights.iter().map(|&w| w as f64).sum();
        let cumulative_weights: Vec<f64> = weights
            .iter()
            .map(|&w| w as f64 / total_weight)
            .scan(0.0, |state, x| {
                *state += x;
                Some(*state)
            })
            .collect();

        Solution { cumulative_weights }
    }

    fn pick_index(&self) -> i32 {
        let rand_num = rand::thread_rng().gen::<f64>();
        match self
            .cumulative_weights
            .binary_search_by(|&w| w.partial_cmp(&rand_num).unwrap())
        {
            Ok(index) => index as i32,
            Err(index) => index as i32,
        }
    }
}

// ```rust
// fn pick_index(&self) -> i32 {
//     let rand_num = rand::thread_rng().gen::<f64>();
//     match self
//         .cumulative_weights
//         .binary_search_by(|&w| w.partial_cmp(&rand_num).unwrap())
//     {
//         Ok(index) => index as i32,
//         Err(index) => index as i32,
//     }
// }
// ```

// 1. **生成随机数**:
//    ```rust
//    let rand_num = rand::thread_rng().gen::<f64>();
//    ```
//    这行代码生成了一个0.0到1.0之间的随机浮点数。它使用 `rand` 包的 `thread_rng()` 函数来获取线程本地的随机数生成器
//    然后调用 `gen::<f64>()` 来生成一个随机的双精度浮点数 (`f64`)。

// 2. **在累积权重中进行二分查找**:
//    ```rust
//    self.cumulative_weights.binary_search_by(|&w| w.partial_cmp(&rand_num).unwrap())
//    ```
//    这里，对 `cumulative_weights` 向量使用了 `binary_search_by` 方法。这个向量被假设为已排序，因为它代表权重的累积分布。二分查找试图在这个排序的向量中找到 `rand_num` 的位置。

//    - `|&w| w.partial_cmp(&rand_num).unwrap()`: 这是一个闭包，用于比较 `cumulative_weights` 中的每个元素 `w` 和 `rand_num`。`partial_cmp` 方法返回一个 `Option<Ordering>`，表示 `w` 相对于 `rand_num` 的比较结果（小于、等于或大于）。`unwrap()` 用于从 `Option` 中提取 `Ordering`。在这里使用 `unwrap()` 是安全的，因为我们知道对于 `f64` 类型，`partial_cmp` 总是会返回 `Some`。

// 3. **处理二分查找的结果**:
//    ```rust
//    match {
//        Ok(index) => index as i32,
//        Err(index) => index as i32,
//    }
//    ```
//    `binary_search_by` 方法返回一个 `Result<usize, usize>`。如果它找到了一个准确的匹配（`Ok`），这意味着 `rand_num` 与累积权重中的某个值完全相等，这种情况虽然罕见，但也是可能的。如果它没有找到准确的匹配（`Err`），它会返回一个值应该插入以保持顺序的索引。

//    在这两种情况下，`index` 都表示 `rand_num` 应该插入 `cumulative_weights` 的位置。这个索引就是根据权重随机选择的索引。`as i32` 是将索引从 `usize`（Rust中通常用于索引的无符号大小类型）转换为 `i32`，以匹配函数的返回类型。

// 这个方法有效地根据权重随机选择了一个索引。选择特定索引的概率对应于原始输入列表中该索引的权重。
