pub fn is_valid(s: String) -> bool {
    let mut st: Vec<char> = Vec::new();

    for l in s.chars() {
        match l {
            '(' | '{' | '[' => {
                st.push(l);
            }
            ']' => {
                if st.pop().or_else(|| Some('a')).unwrap() != '[' {
                    return false;
                }
            }
            '}' => {
                if st.pop().or_else(|| Some('a')).unwrap() != '{' {
                    return false;
                }
            }
            ')' => {
                if st.pop().or_else(|| Some('a')).unwrap() != '(' {
                    return false;
                }
            }
            _ => {}
        }
    }

    st.is_empty()
}

fn main() {
    println!("{}", is_valid("]".to_string()));
}
