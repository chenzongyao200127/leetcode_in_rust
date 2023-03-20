pub fn main() {
    let ans = check_palindrome_formation("khekdvakkkggbopatnbtcbbqkntplzoqectgexifhinhsjohplfebkynpxkraayuythwgbwvzqzprhapxgevfnmexkkutaybuspnmkztgxryipgxlowdsnmqlsslnxupsvsbttxdlgvjvrbxnqezowacfplqkzubwduirbgmjzkdmpwctoowzcsjbaoiumsthvgcagvsihjqgbfjejhtspyrdsmoabvmrgmtshraxgmwknmijgypvgmgfqcytumqcqhgiuihbkcrcehnglsoxegdailsjlibsfnyeoejeltxsvtubakuvskokudtgkbhaab".to_string(),
    "baahbkgtdukoksvukabutvsxtlejeoeynfsbilqwuqnbpyzmlttjzewwcgvcmaqlsosagpztvpbbxkxsclcashgzstktuuernbmymfkfpalyprnmzeakyyruodavblsyxohctqzcknefhucfdpsntixoczieytxeaqaextyeizcoxitnspdfcuhfenkczqtchoxyslbvadouryykaezmnrpylapfkfmymbnreuutktszghsaclcsxkxbbpvtzpgasoslqamcvgcwwezjttlmzypbnquwqhgcqjqzjrfhcfloqdrpvggnupsizifdzeqpvbz".to_string());
    assert_eq!(ans, true);

    let ans = check_palindrome_formation("abc".to_string(), "cba".to_string());
    assert_eq!(ans, true);
}

pub fn check_palindrome_formation(a: String, b: String) -> bool {
    fn check(a: &[u8], b: &[u8], mut left: i32) -> i32 {
        let mut right = a.len() - 1 - left as usize;
        while left >= 0 && right < a.len() {
            if a[left as usize] != b[right] {
                break;
            }
            left -= 1;
            right += 1;
        }
        
        return left
    }

    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut left = (a.len() / 2 - 1) as i32;
    left = check(a, a, left as i32).min(check(b, b, left as i32));
    left = check(a, b, left as i32 ).min(check(b, a, left as i32));

    left == -1
}