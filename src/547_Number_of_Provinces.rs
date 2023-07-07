// 547_Number_of_Provinces
// https://leetcode.cn/problems/number-of-provinces/description/

pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
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
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let cities = is_connected.len();
        let mut uf = UnionFind::new(cities);
        
        for i in 0..cities {
            for j in (i + 1)..cities {
                if is_connected[i][j] == 1 {
                    uf.union(i, j);
                }
            }
        }

        let mut provinces = 0;
        for i in 0..cities {
            if uf.find(i) == i {
                provinces += 1;
            }
        }

        provinces
    }
}

