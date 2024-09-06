
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut un_counter: i32 = nums.len() as i32- 1;

    let mut l: usize = 0;
    let mut aux = 0;

    while l < nums.len() && l <= un_counter as usize{
        if un_counter < 0 || un_counter as usize== l{
            if nums[un_counter as usize] == val {
                un_counter -= 1
            }
            break;
        }
        if nums[l] == val {

            while nums[l] == nums[un_counter as usize] && un_counter as usize > l {
                un_counter -= 1;
            }
            if nums[l] != nums[un_counter as usize] {
                aux = nums[l];
                nums[l] = nums[un_counter as usize];
                nums[un_counter as usize] = aux;
            } else if l == un_counter as usize {
                un_counter -= 1;
                break;
            } else {
                break;
            }
            un_counter -= 1;
            
        }

        l += 1;

    }
    un_counter as i32 + 1
}

fn main() {

    let mut nums = vec![0,1,2,2,3,0,4,2];
    println!("Nums before: {:?}", &nums);

    let a = remove_element(&mut nums, 2);

    println!("Nums after: {:?}", &nums);
    println!("a: {a}");

}
