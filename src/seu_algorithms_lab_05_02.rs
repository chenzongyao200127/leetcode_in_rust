use std::cmp::Ordering;

// 定义船的坐标类型
type Point = (f64, f64);

fn minimum_stations(ships: &mut Vec<Point>, d: f64) -> usize {
    // 按照横坐标排序
    ships.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

    // 初始化基站数量和当前基站的右边界
    let mut stations = 0;
    let mut right_boundary = f64::NEG_INFINITY;

    for ship in ships.iter() {
        // 如果船的位置在当前基站的覆盖范围内，则跳过
        if ship.0 <= right_boundary {
            continue;
        }

        // 计算当前船的基站最右侧位置
        let ship_right_boundary = ship.0 + (d * d - ship.1 * ship.1).sqrt();

        // 更新基站数量和右边界
        stations += 1;
        right_boundary = ship_right_boundary;
    }

    stations
}

fn main() {
    let mut ships = vec![(1.0, 1.0), (3.0, 1.0), (7.0, 1.0), (10.0, 1.0), (14.0, 1.0)];
    let d = 3.0;
    let min_stations = minimum_stations(&mut ships, d);
    println!("最少需要的基站数量: {}", min_stations);
}