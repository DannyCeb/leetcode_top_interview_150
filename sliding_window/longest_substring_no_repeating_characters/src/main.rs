
pub fn no_repeat(s: &Vec<u8>, l: usize, e: usize) -> bool{

    let mut i = l;

    while i <= e {
        for k in l ..=i {
            if s[k] == s[i] && k != i{
                return false;
            }
        }
        i += 1;
    }

    true
}

pub fn length_of_longest_substring(s: String) -> i32 {

    if s.is_empty() {
        return 0
    } 

    let s: Vec<u8> = s.bytes().collect();

    if no_repeat(&s, 0, s.len() - 1) {
        return s.len() as i32;
    }

    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut max_len = 0;
    let mut aux_len = 0;


    while right < s.len() {
        if no_repeat(&s, left, right){
            aux_len = right - left + 1;

            if right == s.len() - 1 {
                if aux_len > max_len {
                    max_len = aux_len
                }
            }

        } else {
            if aux_len > max_len {
                max_len = aux_len
            }
            left+=1;
        }

        right += 1;
        
    }
    max_len as i32
}

fn main() {

    let s = String::from("pwwkew");
    println!("{}",length_of_longest_substring(s));
}
