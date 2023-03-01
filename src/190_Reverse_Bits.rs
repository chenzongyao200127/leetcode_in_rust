// 190. Reverse Bits
// https://leetcode.cn/problems/reverse-bits/

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        if x == 0 {
            return 0 as u32;
        }
        let mut y = 0 as usize;
        for _ in 0..32 {
            if x & 1 == 1 {
                y += 1;
            }
            x >>= 1;
            y <<= 1;
        }

        y
    }
}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        x.reverse_bits()
    }
}

// 对于递归的最底层，我们需要交换所有奇偶位：
// 1.取出所有奇数位和偶数位；
// 2.将奇数位移到偶数位上，偶数位移到奇数位上。
// 类似地，对于倒数第二层，每两位分一组，按组号取出所有奇数组和偶数组，然后将奇数组移到偶数组上，偶数组移到奇数组上。以此类推。\

// class Solution {
//     private:
//         const uint32_t M1 = 0x55555555; // 01010101010101010101010101010101
//         const uint32_t M2 = 0x33333333; // 00110011001100110011001100110011
//         const uint32_t M4 = 0x0f0f0f0f; // 00001111000011110000111100001111
//         const uint32_t M8 = 0x00ff00ff; // 00000000111111110000000011111111
    
//     public:
//         uint32_t reverseBits(uint32_t n) {
//             n = n >> 1 & M1 | (n & M1) << 1;
//             n = n >> 2 & M2 | (n & M2) << 2;
//             n = n >> 4 & M4 | (n & M4) << 4;
//             n = n >> 8 & M8 | (n & M8) << 8;
//             return n >> 16 | n << 16;
//         }
//     };
    
//     作者：LeetCode-Solution
//     链接：https://leetcode.cn/problems/reverse-bits/solution/dian-dao-er-jin-zhi-wei-by-leetcode-solu-yhxz/
//     来源：力扣（LeetCode）
//     著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。