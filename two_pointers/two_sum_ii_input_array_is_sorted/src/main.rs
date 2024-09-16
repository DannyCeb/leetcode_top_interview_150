pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i_i = 0;
    let mut i_f = numbers.len() - 1;

    loop {
        let suma = numbers[i_i] + numbers[i_f];
        if suma == target {
            break;
        } else if suma > target {
            i_f -= 1;
        } else {
            i_i += 1;
        }
    }

    vec![i_i as i32 + 1, i_f as i32 + 1]
}
fn main() {
    println!("{:?}", two_sum(vec![-500, -20, -1, 0, 1, 2], -19));
}
