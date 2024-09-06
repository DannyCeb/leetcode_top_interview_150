
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    for _ in 0 .. k {
        let aux = nums.pop().unwrap();
        nums.insert(0, aux);
    }
}


fn main() {
    let mut nums = vec![-1,-100,3,99];
    rotate(&mut nums, 2);
    println!("{:?}",&nums);
}
