// 1615_Maximal_Network_Rank
// https://leetcode.cn/problems/maximal-network-rank/

// 很慢..但是过了
// 贪心，应该使用邻接矩阵的..
impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        if roads.len() == 0 {
            return 0;
        }

        let mut cnt = vec![0; n as usize];
        for road in roads.iter() {
            for city in road.iter() {
                cnt[*city as usize] += 1;
            }
        }
        let mut cnt: Vec<_> = cnt.iter().enumerate().collect();
        cnt.sort_unstable_by(|a, b| (b.1).cmp(&(a.1)));

        let max_roads_num = cnt[0].1;
        let mut max_citys = vec![];
        for &city in cnt.iter() {
            if city.1 == max_roads_num {
                max_citys.push(city.clone());
            } else {
                break;
            }
        }

        let first = max_citys[0];
        let mut ans = 0;

        if max_citys.len() == 1 {
            let second_roads_num = cnt[1].1;
            let mut second_citys = vec![];
            for &city in cnt.iter().skip(1) {
                if city.1 == second_roads_num {
                    second_citys.push(city.clone());
                } else {
                    break;
                }
            }
            for second in second_citys {
                let mut tmp = first.1 + second.1;
                if roads.contains(&(vec![first.0 as i32, second.0 as i32].clone())) 
                || roads.contains(&(vec![second.0 as i32, first.0 as i32].clone())) {
                    tmp -= 1;
                }
                ans = ans.max(tmp);
            }
            return ans;
        }

        if max_citys.len() == 2 {
            let second = max_citys[1];
            ans = first.1 + second.1;
            if roads.contains(&(vec![first.0 as i32, second.0 as i32].clone())) 
                || roads.contains(&(vec![second.0 as i32, first.0 as i32].clone())) {
                ans -= 1;
            }
            return ans;
        }

        if max_citys.len() >= 2 {
            for i in 0..max_citys.len()-1 {
                for j in i+1..max_citys.len() {
                    let first = max_citys[i];
                    let second = max_citys[j];
                    let mut tmp = first.1 + second.1;
                    if roads.contains(&(vec![first.0 as i32, second.0 as i32].clone())) 
                    || roads.contains(&(vec![second.0 as i32, first.0 as i32].clone())) {
                        tmp -= 1;
                    }
                    ans = ans.max(tmp);
                }
            }
        }

        ans
    }
}

/// 示例代码
/// O(n^2)
impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![0; n]; n];
        let mut degree = vec![0; n];
        for it in roads.iter() {
            let (from, to) = (it[0] as usize, it[1] as usize);
            degree[from] += 1;
            degree[to] += 1;
            adj[from][to] = 1;
            adj[to][from] = 1;
        }
        
        let mut ans = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                ans = ans.max(degree[i] + degree[j] - adj[i][j]);
            }
        }
        ans
    }
}