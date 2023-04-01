// # 331_Verify_Preorder_Serialization_of_a_Binary_Tree
// # https://leetcode.cn/problems/verify-preorder-serialization-of-a-binary-tree/

impl Solution {
    fn is_valid_serialization(preorder: String) -> bool {
        let mut slots = 1;
        let nodes = preorder.split(',');

        for node in nodes {
            slots -= 1;
            if slots < 0 {
                return false;
            }
            if node != "#" {
                slots += 2;
            }
        }

        slots == 0
    }
}


impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut remain = 1;
        for node in preorder.split(',') {
            if remain == 0 {
                return false;
            }
            if node.parse::<i32>().is_err() {
                remain -= 1;
            } else {
                remain += 1;
            }
        }
        remain == 0
    }
}

