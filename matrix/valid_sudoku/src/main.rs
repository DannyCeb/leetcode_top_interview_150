
pub fn is_valid_row(board: &Vec<Vec<char>>, row: usize) -> bool {

    let mut vec_aux: Vec<bool> = vec![false;9];
    let mut resultado = true;

    for c in board[row].iter(){
        if *c != '.' {
            if vec_aux[(*c as u8 - 48) as usize - 1] == true{
                resultado = false;
            } else {
                vec_aux[(*c as u8 - 48) as usize - 1]= true
            }
        }

    }

    resultado
}

pub fn is_valid_column(board: &Vec<Vec<char>>, column: usize) -> bool {
    let mut vec_aux: Vec<bool> = vec![false;9];
    let mut resultado = true;

    for l in 0..9{
        let c = &board[l][column];
        if *c != '.' {
            if vec_aux[(*c as u8 - 48) as usize - 1] == true{
                resultado = false;
            } else {
                vec_aux[(*c as u8 - 48) as usize - 1] = true
            }
        }

    }

    resultado
}

pub fn is_valid_square(board: &Vec<Vec<char>>, row_pos: usize,col_pos: usize) -> bool {
    let mut vec_aux: Vec<bool> = vec![false;9];
    let mut resultado = true;

    let mut l: usize = 0;
    let mut k: usize = 0;

    while l < 3 {
        k = 0;
        while k < 3 {
            let c = &board[l + row_pos][k + col_pos];
            println!("--l:{} -- k:{} -- c:{}--",l,k,c);
            if *c != '.' {
                if vec_aux[(*c as u8 - 48) as usize - 1] == true{
                    resultado = false;
                } else {
                    vec_aux[(*c as u8 - 48) as usize - 1] = true
                }
            }
            k += 1
        }
        l += 1;
    }

    resultado
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {

    for l in 0 .. 9 {
        if !is_valid_row(&board, l) || !is_valid_column(&board, l) {
            return false;
        }
    }

    let v_aux: Vec<usize> = vec![0,3,6];

    for l in &v_aux{
        for k in &v_aux {
            if !is_valid_square(&board, *l, *k) {
                return false;
            }
        }
    }

    true   
}

fn main() {

    let v = vec![
        vec!['.','.','.','.','5','.','.','1','.'],
        vec!['.','4','.','3','.','.','.','.','.'],
        vec!['.','.','.','.','.','3','.','.','1'],
        vec!['8','.','.','.','.','.','.','2','.'],
        vec!['.','.','2','.','7','.','.','.','.'],
        vec!['.','1','5','.','.','.','.','.','.'],
        vec!['.','.','.','.','.','2','.','.','.'],
        vec!['.','2','.','9','.','.','.','.','.'],
        vec!['.','.','4','.','.','.','.','.','.']];
    println!("{}", is_valid_sudoku(v));

    let v: Vec<Vec<char>> = vec!{
        vec!['.','5','.'],
        vec!['3','.','.'],
        vec!['.','.','3'],
    };

    println!("{}",is_valid_square(&v, 0, 0))
}
