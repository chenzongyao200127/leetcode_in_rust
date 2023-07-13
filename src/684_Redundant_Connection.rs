pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
        }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x] = root_y;
        }
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();

        let mut uf = UnionFind::new(n);

        let mut redundant = vec![];

        for edge in edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;

            let parent_x = uf.find(x);
            let parent_y = uf.find(y);

            if parent_x != parent_y {
                uf.union(parent_x, parent_y);
            } else {
                redundant = vec![x as i32, y as i32];
                break;
            }
        }

        redundant
    }
}

// 并查集是一种树形的数据结构，常用于处理一些不相交集合的合并及查询问题。它有两个主要的操作，即：合并（Union）和查找（Find）。

// 合并（Union）：将两个子集合并成同一个集合。一般来说，我们通过把一个集合的代表元素（根节点）作为另一集合的父节点来实现。

// 查找（Find）：确定元素属于哪一个子集。它可以被用来确定两个元素是否属于同一子集。

// 以上面的 Rust 代码为例，find 函数是查找函数，它用于查找给定节点的根节点；union 函数是合并函数，它用于将两个节点的根节点进行合并。

// 并查集还有一个重要的优化策略，叫做“路径压缩”，即在执行“查找”操作的时候，
// 让查找路径上的每个节点直接连接到根节点，这样可以降低后续查找操作的复杂度。在这段代码中，find 函数的以下部分就实现了路径压缩：

// if parent[x] as usize != x {
//     parent[x] = find(parent, parent[x] as usize) as i32;
// }

// 总的来说，合并-查找集合是解决某些算法问题（如网络连通性、等价类问题等）的有效工具，因为它可以高效地进行合并操作和查找操作。
