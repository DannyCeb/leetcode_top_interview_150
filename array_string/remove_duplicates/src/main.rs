


pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut aux_map: HashMap<i32, u8> = HashMap::new();
    let mut auxiliar = nums.len();
    let mut l: usize = 0;
    while l < auxiliar    {
        //println!("nums: {:?}",nums);
        match aux_map.contains_key(&nums[l]) {
            true => {
                match aux_map.get(&(nums[l] as i32)){
                    Some(v) => {
                        if *v == 1{
                            *aux_map.entry(nums[l] as i32).or_insert(0) += 1;
                            l += 1;

                        } else {
                            nums.remove(l);
                            auxiliar = nums.len();
                        }
                    }
                    _ => {}
                }
                
            }
            false =>{
                aux_map.insert(nums[l] as i32, 1);
                l += 1;
            }
        }
        
    }

    //println!("map: {:?}", aux_map);

    nums.len() as i32

}

fn main() {
    let mut nums = vec![0,0,1,1,1,1,2,3,3];

    println!("v before:{:?}",&nums);

    let k = remove_duplicates(&mut nums);

    println!("k {}", k);
    println!("v after: {:?}",&nums);
}
