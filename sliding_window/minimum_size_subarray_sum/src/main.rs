use std::cmp;

    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        
        let mut min_array_len = usize::MAX;
        let mut curr_running_sum = 0;

        let mut left_ptr = 0;
        for right_ptr in 0..nums.len(){
            
            curr_running_sum += nums[right_ptr];
            while curr_running_sum >= target{

                min_array_len = cmp::min(min_array_len, right_ptr  - left_ptr + 1);
                curr_running_sum -= nums[left_ptr];
                left_ptr += 1;
            }
        }


        if min_array_len == usize::MAX{
            0
        }else{
            min_array_len as i32
        }    
    } 
    


fn main() {
    //println!("res: {}",min_sub_array_len(213, vec![12,28,83,4,25,26,25,2,25,25,25,12]));

    println!("res: {}",min_sub_array_len(7, vec![2,3,1,2,4,3]));

    //println!("res: {}",min_sub_array_len(15, vec![5,1,3,5,10,7,4,9,2,8]));
}
