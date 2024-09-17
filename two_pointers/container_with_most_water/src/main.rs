fn calcular_area(v: &Vec<i32>, i_i: usize, i_f: usize) -> i32 {
    std::cmp::min_by(v[i_i], v[i_f], |a, b| a.cmp(b)) * (i_f - i_i) as i32
}

pub fn max_area_n2(height: Vec<i32>) -> i32 {
    let mut area = 0;

    for l in 0..height.len() {
        for k in l..height.len() {
            let aux_area = calcular_area(&height, l, k);
            if aux_area > area {
                area = aux_area;
            }
        }
    }

    area
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut area = 0;

    let mut i_i = 0;
    let mut i_f = height.len() - 1;

    while i_i != i_f {
        let aux_area = calcular_area(&height, i_i, i_f);
        if aux_area > area {
            area = aux_area
        }

        if height[i_i] < height[i_f] {
            i_i += 1;
        } else {
            i_f -= 1;
        }
    }

    area
}

fn main() {
    println!("{}", max_area(vec![2, 3, 4, 5, 18, 17, 6]));
}
