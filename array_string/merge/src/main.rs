pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut aux: Vec<i32> = vec!();
         let mut count: usize = 0;
    let mut count2: usize = 0;
    let mut auxiliar: String = String::from("Danny");

    loop {
        if count < m as usize&& count2 < n  as usize{
            let a = nums1.get(count ).unwrap();
            let b = nums2.get( count2 ).unwrap();
            if a < b {
                aux.push( *a );
                count += 1;
            } else {
                aux.push( *b );
                count2 += 1;
            }
        } else if count < m as usize {
            let a = nums1.get(count ).unwrap();
            aux.push( *a );
            count += 1;
        } else if count2 < n as usize{
            let b = nums2.get( count2 ).unwrap();
            aux.push( *b );
            count2 += 1;
        } else {
            break;
        }
    }

    println!("Aux:{:?}", aux);

    *nums1 = aux.clone();
}

fn main() {
    /*
        nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
    
     */

    let mut nums1 = vec![1,2,3,0,0,0];
    let mut nums2 = vec![2,5,6];

    println!("V1 before: {:?}",&nums1);

    merge(&mut nums1, 3, &mut nums2, 3);

    println!("v1 after: {:?}",&nums1)

}

