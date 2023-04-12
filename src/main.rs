fn main() {
    // let ans = longest_decomposition("antaprezatepzapreanta".to_string());
    // assert_eq!(ans, 11);

    // let ans = longest_decomposition("ghiabcdefhelloadamhelloabcdefghi".to_string());
    // assert_eq!(ans, 7);

    // let ans = longest_decomposition("merchant".to_string());
    // assert_eq!(ans, 1);

    let ans = longest_decomposition("elvtoelvto".to_string());
    assert_eq!(ans, 2);
}

pub fn longest_decomposition(text: String) -> i32 {
    let mut left = 1;
    let mut prev_left = 0;
    let mut right = text.len() - 1;
    let mut pos_right = text.len();

    // let text: Vec<char> = text.chars().collect();

    let mut ans = 1;

    while left <= right {
        println!("{:?}", (left, right));
        println!("{:?}", (&text[prev_left..left], &text[right..pos_right]));
        if text[prev_left..left] == text[right..pos_right] && left <= right {
            if left == right {
                ans += 1;
                prev_left = left;
                pos_right = right;
                left += 1;
                right -= 1;
            } else {
                ans += 2;
                prev_left = left;
                pos_right = right;
                left += 1;
                right -= 1;
            }
        } else {
            left += 1;
            right -= 1;
        }
    }

    ans
}