use std::collections::HashMap;


pub fn create_hash(s: &String) -> HashMap<char,u64> {
    let mut hs: HashMap<char,u64> = HashMap::new();

    let _ : Vec<()>= s.chars().map(|c| {
        match hs.get_mut(&c) {
            Some(caracter) => *caracter += 1,
            None => {hs.insert(c, 1);}
        }
    }).collect();
    hs
}

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    
    if magazine.len() < ransom_note.len() {
        return false;
    }

    let hr = create_hash(&ransom_note);
    let hm = create_hash(&magazine);


    for (k,v) in hr {
        match hm.get(&k) {
            Some(n) => {
                if *n < v {
                    return false;
                }
            },
            None => {
                return false;
            }
        }
    }
    

    true
}

fn main() {
    println!("{}",can_construct("".to_string(), "".to_string()));

}
