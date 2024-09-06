pub fn is_subsequence(s: String, t: String) -> bool {
    let mut counter: usize = 0;
    let longer:Vec<u8> = t.bytes().collect();
    let short:Vec<u8> = s.bytes().collect();

    if !s.is_empty() {
        for l in 0 .. s.len(){
            while counter < longer.len() {
                counter +=1;
                if longer[counter - 1] == short[l] && l == short.len() - 1{
                    return true;
                } else if longer[counter - 1] == short[l] {
                    break;
                }
            }
        }
        return false;
    }

    true
    
}

fn main() {
    println!("{}",is_subsequence("axc".to_string(), "ahbgdc".to_string()));
}
