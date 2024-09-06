pub fn is_palindrome(s: String) -> bool {
    
    let mut s: Vec<u8> = s.to_lowercase().bytes().filter_map(|x| {
        match x {
            97..=122 | 48..=57 => {
                Some(x)
            }
            _ => None
        }
    }).collect();

    let s2 = s.clone();
    s.reverse();

    s2 == s
}

fn main() {
    println!("{}",is_palindrome("A man, a plan, a canal: Panama".to_string()));
}
