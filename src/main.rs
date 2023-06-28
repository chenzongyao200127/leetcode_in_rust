pub fn minimum_incompatibility(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let group_size = n / k as usize;
    if n % k as usize != 0 {
        return -1;
    }
    nums.sort_unstable();
    let mut groups = vec![vec![]; k as usize];
    let mut min_incompatibility = i32::MAX;

    fn backtrack(
        nums: &[i32],
        groups: &mut Vec<Vec<i32>>,
        k: usize,
        group_size: usize,
        incompatibility: i32,
        min_incompatibility: &mut i32,
    ) {
        if nums.is_empty() {
            *min_incompatibility = i32::min(*min_incompatibility, incompatibility);
            return;
        }
        
        if incompatibility >= *min_incompatibility {
            return;
        }

        let num = nums[0];
        let remaining_nums = &nums[1..];
        for i in 0..k {
            if groups[i].len() < group_size && !groups[i].contains(&num) {
                let prev_incompatibility = if groups[i].is_empty() {
                    0
                } else {
                    num - groups[i][0]
                };
                groups[i].push(num);
                backtrack(
                    remaining_nums,
                    groups,
                    k,
                    group_size,
                    incompatibility + prev_incompatibility,
                    min_incompatibility,
                );
                groups[i].pop();
            }
        }
    }

    backtrack(&nums, &mut groups, k as usize, group_size, 0, &mut min_incompatibility);
    if min_incompatibility == i32::MAX {
        -1
    } else {
        min_incompatibility
    }
}

fn main() {
    println!("{:?}", minimum_incompatibility(vec![7,3,16,15,1,13,1,2,14,5,3,10,6,2,7,15], 8))
}