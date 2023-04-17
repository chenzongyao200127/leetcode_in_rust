
fn main() {

    let ans = count_days_together(
    "08-15".to_string(),
    "08-18".to_string(),
    "08-16".to_string(),
    "08-16".to_string(),
    );

    assert_eq!(ans, 1);
}

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

    println!("{:?}", (alice_arrive_unidate,alice_leave_unidate,bob_arrive_unidate,bob_leave_unidate));

    let ans = alice_leave_unidate.min(bob_leave_unidate)
        - alice_arrive_unidate.max(bob_arrive_unidate);

    if ans < 0 {
        return 0;
    }

    ans + 1
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