use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::from(nums);
    for _ in 0..k - 1 {
        heap.pop();
    }
    heap.pop().unwrap()
}

fn main() {
    println!("{}", find_kth_largest(vec![-1, 2, 0], 3));
}
