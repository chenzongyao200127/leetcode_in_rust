// 2594. Minimum Time to Repair Cars
// https://leetcode.cn/problems/minimum-time-to-repair-cars/description/?envType=daily-question&envId=2023-09-07

impl Solution {
    pub fn repair_cars(mut ranks: Vec<i32>,cars: i32) -> i64 {
        ranks.sort();
        let cars=cars as i64;
        let mut l=1_i64;
        let mut r=9223372036854775807_i64;

        while l<r {
            let mid=((r-l)>>1)+l;
            let mut top=false;
            let mut t=0_i64;
            for v in &ranks {
                t+=((mid/(*v as i64))as f64).sqrt() as i64;
                if t>=cars{
                    top=true;
                    break;
                }
            }
            if top{
                r=mid;
            }else{
                l=mid+1;
            }
        }
        
        l
    }
}