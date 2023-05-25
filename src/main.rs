use std::collections::HashMap;

fn main() {

}


pub fn odd_string(words: Vec<String>) -> String {
    let mut cnt = HashMap::new();
    for s in words.iter() {
        let k = s.as_bytes()
                            .windows(2)
                            .map(|v| (v[1] as i32 - v[0] as i32)
                            .to_string() + ",")
                            .collect::<String>();
        let v = cnt.entry(k).or_insert(Vec::new());
        v.push(s);
    }
    cnt.values().find(|v| v.len() == 1).unwrap()[0].clone()
}