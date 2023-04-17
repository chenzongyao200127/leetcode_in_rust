// 2409_Count_Days_Spent_Together
// https://leetcode.cn/problems/count-days-spent-together/

impl Solution {
    pub fn count_days_together(arrive_alice: String, leave_alice: String, arrive_bob: String, leave_bob: String) -> i32 {
        let arrive_alice_moth = arrive_alice[0..2].parse::<i32>().unwrap();
        let arrive_alice_date = arrive_alice[3..5].parse::<i32>().unwrap();

        let leave_alice_moth = leave_alice[0..2].parse::<i32>().unwrap();
        let leave_alice_date = leave_alice[3..5].parse::<i32>().unwrap();

        let arrive_bob_moth = arrive_bob[0..2].parse::<i32>().unwrap();
        let arrive_bob_date = arrive_bob[3..5].parse::<i32>().unwrap();

        let leave_bob_moth = leave_bob[0..2].parse::<i32>().unwrap();
        let leave_bob_date = leave_bob[3..5].parse::<i32>().unwrap();

        let alice_arrive_unidate = unidates(arrive_alice_moth, arrive_alice_date);
        let alice_leave_unidate = unidates(leave_alice_moth, leave_alice_date);
        let bob_arrive_unidate = unidates(arrive_bob_moth, arrive_bob_date);
        let bob_leave_unidate = unidates(leave_bob_moth, leave_bob_date);

        let ans = alice_leave_unidate.min(bob_leave_unidate)
            - alice_arrive_unidate.max(bob_arrive_unidate);

        if ans < 0 {
            return 0;
        }

        ans + 1
    }
}


pub fn unidates(moth: i32, date: i32) -> i32 {
    let month_dates = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut res = 0;
    for i in 1..month {
        res += month_dates[i as usize - 1];
    }
    for _ in 1..=date {
        res += 1;
    }

    res
}



impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let (arrive_alice_month, arrive_alice_date) = parse_date(&arrive_alice);
        let (leave_alice_month, leave_alice_date) = parse_date(&leave_alice);
        let (arrive_bob_month, arrive_bob_date) = parse_date(&arrive_bob);
        let (leave_bob_month, leave_bob_date) = parse_date(&leave_bob);

        let alice_arrive_unidate = unique_dates(arrive_alice_month, arrive_alice_date);
        let alice_leave_unidate = unique_dates(leave_alice_month, leave_alice_date);
        let bob_arrive_unidate = unique_dates(arrive_bob_month, arrive_bob_date);
        let bob_leave_unidate = unique_dates(leave_bob_month, leave_bob_date);

        let ans = alice_leave_unidate.min(bob_leave_unidate)
            - alice_arrive_unidate.max(bob_arrive_unidate);

        if ans < 0 {
            return 0;
        }

        ans + 1
    }
}

fn parse_date(date_str: &str) -> (i32, i32) {
    let date_parts: Vec<&str> = date_str.split('-').collect();
    (
        date_parts[0].parse::<i32>().unwrap(),
        date_parts[1].parse::<i32>().unwrap(),
    )
}

fn unique_dates(month: i32, date: i32) -> i32 {
    let month_dates = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut res = 0;
    for i in 1..month {
        res += month_dates[i as usize - 1];
    }
    for _ in 1..=date {
        res += 1;
    }

    res
}


use std::{cmp::max, cmp::min};
static MONTH_DAYS: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
fn count_days(date: &str) -> i32 {
    let month = date[0..2].parse::<usize>().unwrap();
    let day = date[3..5].parse::<i32>().unwrap();
    MONTH_DAYS[month-1]+day
}
impl Solution {
    fn count_days_together(arrive_alice: String, leave_alice: String, arrive_bob: String, leave_bob: String) -> i32 {
        let aa = count_days(&arrive_alice);
        let la = count_days(&leave_alice);
        let ab = count_days(&arrive_bob);
        let lb = count_days(&leave_bob);
        let intersect_l = max(aa, ab);
        let intersect_r = min(la, lb);
        if intersect_l > intersect_r {
            return 0;
        }
        intersect_r-intersect_l+1
    }
}