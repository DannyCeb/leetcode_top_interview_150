pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut res: Vec<String> = vec![];

    let mut inicio: usize = 0;
    let mut fin: usize = 0;

    while fin < nums.len() {
        if fin == nums.len() - 1 {
            if fin == inicio {
                res.push(format!("{}", nums[fin]));
            } else {
                res.push(format!("{}->{}", nums[inicio], nums[fin]));
            }
        } else if nums[fin] != nums[fin + 1] - 1 {
            if fin == inicio {
                res.push(format!("{}", nums[fin]));
            } else {
                res.push(format!("{}->{}", nums[inicio], nums[fin]));
            }

            inicio = fin + 1;
        }

        fin += 1;
    }

    res
}

fn main() {
    println!("{:?}", summary_ranges(vec![0, 1, 2, 4, 5, 7]));
}
