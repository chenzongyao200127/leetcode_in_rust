// 831_Masking_Personal_Information
// https://leetcode.cn/problems/masking-personal-information/

impl Solution {
    pub fn mask_pii(s: String) -> String {
        if let Some(idx) = s.find('@') {
            let s = s.to_ascii_lowercase();
            let mut new_s = s[0..1].to_string();

            new_s.push_str(&"*****");
            new_s.push_str(&s[idx-1..s.len()]);

            return new_s
        } else {
            let splits = vec!['+', '-', '(', ')', ' '];

            let s: Vec<char> = s.chars().collect();
            let mut new_s = vec![];

            s.iter().for_each(|ch| {
                if !splits.contains(ch) {
                    new_s.push(*ch)
                }
            });

            let mut ans = "".to_string();

            match new_s.len() {
                10 => ans.push_str(&"***-***-"),
                11 => ans.push_str(&"+*-***-***-"),
                12 => ans.push_str(&"+**-***-***-"),
                _  => ans.push_str(&"+***-***-***-"),
            }

            for i in new_s.len()-4..new_s.len() {
                ans.push(new_s[i]);
            }

            return ans;
        }
    }
}



impl Solution {
    pub fn handle_email(s: &str) -> String {
        let lower = s.to_lowercase();
        let vec = lower.split('@').collect::<Vec<&str>>();
        if let [name, domain] = &vec[..] {
            println!("name: {}, domain: {}", name, domain);
            let first_letter = name.chars().nth(0).unwrap();
            let last_letter = name.chars().nth(name.len() - 1).unwrap();
            return format!("{}*****{}@{}", first_letter, last_letter, domain);
        } else {
            return s.to_string();
        }
    }

    pub fn handle_phone_number(s: &str) -> String {
        let numbers = s.replace(&['+', '-', '(', ')', ' '][..], "");
        println!("numbers: {}", numbers);
        let len = numbers.len();
        let mut country_str = String::from("");
        if len > 10 {
            country_str = format!("+{}-", "*".repeat(len - 10).as_str());
        }
        println!("country_str: {}", country_str);
        let last_number: String = numbers.chars().skip(len - 4).take(4).collect();
        format!("{}***-***-{}", country_str, last_number)
    }

    pub fn mask_pii(s: String) -> String {
        if s.contains('@') && s.contains('.') {
            Self::handle_email(&s)
        } else {
            Self::handle_phone_number(&s)
        }
    }
}


impl Solution {
    pub fn mask_pii(s: String) -> String {
        const COUNTRY_CODE: [&str; 4] = ["", "+*-", "+**-", "+***-"];
        match s.find('@') {
            Some(idx) => {
                s[0..1].to_lowercase() + "*****" + &s[(idx - 1)..].to_lowercase()
            },
            None => {
                let numbers = s.matches(char::is_numeric).collect::<String>();
                let n = numbers.len();
                COUNTRY_CODE[n - 10].to_string() + "***-***-" + &numbers[(n - 4)..]
            }
        }
    }
}
