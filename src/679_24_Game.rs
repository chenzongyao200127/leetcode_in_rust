// 679_24_Game
// https://leetcode.cn/problems/24-game/description/

use itertools::Itertools;
use permutohedron::Heap;
use std::f64;

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn apply(&self, a: f64, b: f64) -> Option<f64> {
        match self {
            Op::Add => Some(a + b),
            Op::Sub => Some(a - b),
            Op::Mul => Some(a * b),
            Op::Div => if b != 0.0 { Some(a / b) } else { None },
        }
    }
}

fn judge_point24(cards: &[f64; 4]) -> bool {
    let mut nums = cards.clone();
    let heap = Heap::new(&mut nums);

    let ops = [Op::Add, Op::Sub, Op::Mul, Op::Div];
    let op_tuples = 
        ops.iter().cartesian_product(ops.iter()).cartesian_product(ops.iter());

    for perm in heap {
        println!("{:?}", perm);
        for ((op1, op2), op3) in op_tuples.clone() {
            if let Some(res) = op1.apply(op2.apply(perm[0], perm[1]).unwrap_or(f64::NAN), op3.apply(perm[2], perm[3]).unwrap_or(f64::NAN)) {
                if (res - 24.0).abs() < 1e-6 {
                    return true;
                }
            }
            if let Some(res) = op1.apply(perm[0], op2.apply(op3.apply(perm[1], perm[2]).unwrap_or(f64::NAN), perm[3]).unwrap_or(f64::NAN)) {
                if (res - 24.0).abs() < 1e-6 {
                    return true;
                }
            }
            if let Some(res) = op1.apply(perm[0], op2.apply(perm[1], op3.apply(perm[2], perm[3]).unwrap_or(f64::NAN)).unwrap_or(f64::NAN)) {
                if (res - 24.0).abs() < 1e-6 {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let cards = [4 as f64, 1 as f64, 8 as f64, 7 as f64];
    println!("{}", judge_point24(&cards));
}
