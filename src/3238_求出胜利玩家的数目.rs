use std::collections::HashMap;

// 给你一个整数 n ，表示在一个游戏中的玩家数目。同时给你一个二维整数数组 pick ，其中 pick[i] = [xi, yi] 表示玩家 xi 获得了一个颜色为 yi 的球。

// 如果玩家 i 获得的球中任何一种颜色球的数目 严格大于 i 个，那么我们说玩家 i 是胜利玩家。换句话说：

// 如果玩家 0 获得了任何的球，那么玩家 0 是胜利玩家。
// 如果玩家 1 获得了至少 2 个相同颜色的球，那么玩家 1 是胜利玩家。
// ...
// 如果玩家 i 获得了至少 i + 1 个相同颜色的球，那么玩家 i 是胜利玩家。
// 请你返回游戏中 胜利玩家 的数目。

// 注意，可能有多个玩家是胜利玩家。

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut count: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        for p in pick {
            let player = p[0];
            let color = p[1];
            let entry = count.entry(player).or_insert_with(HashMap::new);
            *entry.entry(color).or_insert(0) += 1;
        }
        let mut res = 0;
        for (player, colors) in count {
            for &c in colors.values() {
                if c > player {
                    res += 1;
                    break;
                }
            }
        }
        res
    }
}
