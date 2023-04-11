// 1041_Robot_Bounded_In_Circle
// https://leetcode.cn/problems/robot-bounded-in-circle/

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        // direction: '北': 0 '西': 1 '南': 2, '东': 3
        let mut location = ((0, 0), 0);
        let instructions: Vec<char> = instructions.chars().collect();

        for &ins in instructions.iter() {
            match ins {
                'G' => {
                    match location.1 {
                        0 => {
                            location.0.1 += 1;
                        },
                        1 => {
                            location.0.0 -= 1;
                        }
                        2 => {
                            location.0.1 -= 1;
                        }
                        _ => {
                            location.0.0 += 1;
                        }
                    }
                }
                'L' => {
                    location.1 = (location.1 + 5) % 4;
                },
                _  => {
                    location.1 = (location.1 + 3) % 4;
                }
            }
        }

        location.0 == (0, 0) || location.1 != 0
    }
}


impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let (mut dir, mut x, mut y) = (0, 0, 0);
        for i in instructions.bytes() {
            match i {
                b'L' => dir = (dir + 4 - 1) % 4,
                b'R' => dir = (dir + 1) % 4,
                _ => match dir {
                    0 => y += 1,
                    1 => x += 1,
                    2 => y -= 1,
                    _ => x -= 1,
                },
            }
        }
        !(dir == 0 && (x, y) != (0, 0))
    }
}


// 先忽略位移，只考虑旋转，由于旋转基元是90°，所以每次instruction导致的角位移一定是90°的生成旋转群元素之一，
// 这显然是个四阶群，所以群中每个元素的四次幂一定是零元，
// 说人话就是做四遍instruction一定会脸朝北，所以只要模拟四次看看位置是不是在原地就行。
// class Solution:
//     def isRobotBounded(self, instructions: str) -> bool:
//         dirs = ((0, 1), (1, 0), (0, -1), (-1, 0))
//         d = x = y = 0

//         def do(command):
//             nonlocal x, y, d
//             if command == 'G':
//                 dx, dy = dirs[d]
//                 x += dx
//                 y += dy
//             elif command == 'L':
//                 d = (d + 3) % 4
//             else:  # if command == 'R':
//                 d = (d + 1) % 4
        
//         for _ in range(4):
//             for c in instructions:
//                 do(c)
        
//         return x == y == 0