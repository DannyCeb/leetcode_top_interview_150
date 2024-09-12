pub fn max_profit(prices: Vec<i32>) -> i32 {
    use std::cmp::{max, min};
    let mut low = i32::MAX;
    let mut result = 0;

    for i in prices {
        result = max(result, i - low);
        low = min(low, i);
    }

    result
}

fn main() {
    let prices = vec![7, 99, 1, 5, 3, 6, 4, 10];
    println!("{}", max_profit(prices));
}
