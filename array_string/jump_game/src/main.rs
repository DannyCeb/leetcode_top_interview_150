pub fn c_j(nums: &Vec<i32>, index: usize) -> bool {
    if index == nums.len() - 1 {
        true
    } else if nums[index] == 0 {
        false
    } else {
        let steps = nums[index];
        for l in 1..=steps {
            if c_j(&nums, index + l as usize) {
                return true;
            }
        }
        false
    }
}

pub fn can_jump_r(nums: Vec<i32>) -> bool {
    c_j(&nums, 0)
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }

    let mut index: usize = 0;

    while index < nums.len() {
        let steps: usize = nums[index] as usize;
        if steps == 0 {
            return false;
        }
        //let aux_count:usize = 1;

        if steps + index >= (nums.len() - 1) {
            return true;
        }
        let slic = &nums[(index + 1)..=(index + steps)];
        let aux = slic
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap();

        if slic[aux] == 0 {
            return false;
        }
        index += aux + 1;

        //index += steps;
    }

    true
}

fn main() {
    println!("{}", can_jump_r(vec![4, 2, 0, 0, 1, 1, 4, 4, 4, 0, 4, 0]));
}
