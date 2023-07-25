// 2208_Minimum_Operations_to_Halve_Array_Sum
// https://leetcode.cn/problems/minimum-operations-to-halve-array-sum/

// 给你一个正整数数组 nums 。每一次操作中，你可以从 nums 中选择 任意 一个数并将它减小到 恰好 一半。（注意，在后续操作中你可以对减半过的数继续执行操作）
// 请你返回将 nums 数组和 至少 减少一半的 最少 操作数。

use std::cmp::Ordering;
use std::collections::BinaryHeap;

// 在这段代码中，我们创建了一个新的类型 `ComparableF64`，它是 `f64` 类型的包装器（wrapper）。
// 我们这样做的目的是为了能够在 `f64` 上使用某些特性（trait）来满足某些 API 的要求。

// 在这种情况下，我们想要使用 `std::collections::BinaryHeap`，它需要元素类型实现 `Ord` trait，以便知道如何对元素进行排序。
// 然而，`f64` 类型并没有实现 `Ord`，这是因为浮点数有一个特殊的值 `NaN`（Not a Number），
// `NaN` 既不大于、也不小于任何数，也不等于任何数，包括它自身。因此，为了让我们可以将 `f64` 类型的值存入 `BinaryHeap`，
// 我们需要创建一个新的类型 `ComparableF64` 并为其实现 `Ord`。

// 这就是 `ComparableF64` 结构体和下面的四个 `impl` 块的作用。

// - `PartialEq`：用于检查两个数是否相等。我们的实现方法是简单地比较两个数的 `f64` 部分是否相等。
// 如果两者的 `f64` 部分相等，我们就认为这两个 `ComparableF64` 是相等的。

// - `Eq`：这是 `PartialEq` 的 "升级版"，它表明每一对元素都是可比较的，也就是说，没有一个元素既不等于也不不等于另一个元素。
// 因为我们知道我们将不会用 `NaN` 值来创建 `ComparableF64`，所以我们可以放心地实现这个 trait。

// - `PartialOrd`：这是一个用于比较两个数的 trait。我们的实现方法是简单地比较两个数的 `f64` 部分。
// 如果 `self` 的 `f64` 部分小于 `other` 的 `f64` 部分，那么 `self` 就小于 `other`，依此类推。

// - `Ord`：这是 `PartialOrd` 的 "升级版"，它表明每一对元素都是可比较的。
// 在我们的实现中，如果 `self.partial_cmp(other)` 返回 `None`，我们就返回 `Ordering::Equal`。
// 但是，这只会在 `self` 和 `other` 的 `f64` 部分中至少有一个是 `NaN` 时发生，
// 由于我们不会使用 `NaN` 值来创建 `ComparableF64`，所以我们实际上可以放心地 `unwrap` 这个 `Option`。

// 这样，我们就能够使用 `ComparableF64` 类型的值来创建一个二进制堆（`BinaryHeap<ComparableF64>`）了，
// 因为 `ComparableF64` 满足了二进制堆所需的 `Ord` trait。

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct ComparableF64(f64);

impl PartialEq for ComparableF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ComparableF64 {}

impl PartialOrd for ComparableF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for ComparableF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut accum = 0.0;
        let mut max_heap = BinaryHeap::new();
        let total = nums.iter().map(|x| *x as f64).sum::<f64>();
        for n in nums {
            max_heap.push(ComparableF64(n as f64));
        }
        let mut cnt = 0;
        while accum < total / 2.0 {
            if let Some(ComparableF64(tmp_max)) = max_heap.pop() {
                accum += tmp_max / 2.0;
                cnt += 1;
                max_heap.push(ComparableF64(tmp_max / 2.0));
            } else {
                break;
            }
        }
        cnt
    }
}



// 堆的使用方法


fn main() {
    // 使用 Reverse 包装元素，让 BinaryHeap 变成最大堆
    let mut max_heap = BinaryHeap::new();
    max_heap.push(Reverse(1));
    max_heap.push(Reverse(5));
    max_heap.push(Reverse(2));

    // 注意取出的时候需要再次解包
    assert_eq!(max_heap.pop(), Some(Reverse(5)));
    assert_eq!(max_heap.pop(), Some(Reverse(2)));
    assert_eq!(max_heap.pop(), Some(Reverse(1)));
}


// HashMap 和堆是计算机科学中的两种基本数据结构，它们的实现原理在很多编程语言和框架中都是相似的。下面是这两种数据结构在底层的一些基本实现原理。

// **1. HashMap**

// HashMap是基于哈希表的数据结构。在其最简单的形式中，哈希表是一个数组，数组的每个元素（也称为“槽”或“bucket”）包含一个键值对的链表。
// 以下是HashMap的基本操作的实现方式：

// - *插入操作*：首先，对键应用哈希函数，以确定应将键值对插入哪个桶。然后，将键值对添加到该桶中。如果在该桶中已存在相同的键，则更新其值。
// - *查找操作*：对键应用哈希函数，以确定在哪个桶中查找键值对。然后，遍历该桶中的链表，查找与给定键匹配的节点。
// - *删除操作*：与查找操作类似，只是找到匹配的节点后，将其从链表中删除。

// 为了防止哈希冲突（即多个键映射到同一桶）导致性能下降，一旦负载因子（即元素数除以桶数）超过某个阈值，哈希表就会进行重哈希，以增加桶的数量。

// **2. 堆（Heap）**

// 堆通常通过数组来实现，这种数组满足堆特性，即对于每个元素 `i`，其子元素在数组中的位置为 `2i+1` 和 `2i+2`，其父元素的位置为 `(i-1)/2`。

// - *插入操作*：新元素被添加到数组的末尾，然后进行"上浮"操作，将这个元素与它的父元素比较，如果它比父元素大（或小，在最小堆中），则将它与父元素交换。
// 这个过程一直持续到堆特性得到满足。

// - *删除操作*：通常是删除最大元素（或最小元素，在最小堆中）。
// 首先，将堆顶元素与数组的最后一个元素交换，然后从数组中删除最后一个元素。
// 然后，执行"下沉"操作，将新的堆顶元素与其子元素进行比较，
// 如果它小于任何一个子元素（或大于，在最小堆中），则与较大（或较小）的子元素进行交换。这个过程一直持续到堆特性得到满足。

// 这些只是基本的实现方式，不同的编程语言或框架可能会有一些改进或优化，以提高性能或内存使用效率。
// 例如，一些哈希表实现使用开放寻址来处理哈希冲突，而不是链表。一些堆的实现（如斐波那契堆）可以更有效地执行某些操作。


// 在 Rust 中，`PartialEq`, `Eq`, `PartialOrd` 和 `Ord` 这四个 trait 用于比较和排序。下面是每个 trait 的详细介绍：

// - `PartialEq`: 用于比较两个值是否相等。这个 trait 的方法 `eq` 返回一个布尔值，表示 self 是否等于另一个给定的值。
// 几乎所有类型都实现了 `PartialEq` trait，因为它是由 derive 宏自动实现的。例如：

//   ```rust
//   let x = 5;
//   let y = 10;
//   assert_eq!(x.eq(&y), false);
//   ```

// - `Eq`: 是 `PartialEq` 的强化版本。`PartialEq` 对比较进行了一些限制，例如浮点数由于存在 NaN 值，其比较并非全序关系，
// 只能实现 `PartialEq`，不能实现 `Eq`。而像整数这样的全序类型，则可以实现 `Eq`。`Eq` 是没有任何方法的标记（marker）trait，
// 它用来表示类型的每一对值都是可以比较的。例如：

//   ```rust
//   let x = 5;
//   let y = 10;
//   assert_eq!(x == y, false);
//   ```

// - `PartialOrd`: 允许对类型的值进行顺序比较。它有一个方法 `partial_cmp`，返回一个 `Option<Ordering>`。
// 这个返回值可能是 `Some(Less)`, `Some(Greater)`, `Some(Equal)` 或 `None`。
// 对于那些并非全序关系的类型，例如浮点数（因为存在 NaN），可以实现 `PartialOrd`。例如：

//   ```rust
//   let x = 5.0;
//   let y = 10.0;
//   assert_eq!(x.partial_cmp(&y), Some(Ordering::Less));
//   ```

// - `Ord`: 是 `PartialOrd` 的强化版本。它需要类型的值能构成全序关系。
// 它有一个方法 `cmp`，返回一个 `Ordering`。可以用 `Ord` trait 来排序一个序列。例如：

//   ```rust
//   let mut vec = vec![5, 2, 1];
//   vec.sort();  // 需要 Ord trait
//   assert_eq!(vec, vec![1, 2, 5]);
//   ```

// 这四个 trait 之间的主要区别在于他们支持的操作和适用的类型。`PartialEq` 和 `PartialOrd` 支持任意类型，
// 但是他们的方法可能会返回 `None`，表示值之间不能进行比较。`Eq` 和 `Ord` 只适用于那些可以全序比较的类型，他们的方法总是能给出一个确定的结果。