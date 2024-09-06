
#[derive(Clone, Copy)]
pub struct  Coords {
    row: usize,
    col: usize
}
 
impl Coords {
    pub fn new (row: usize, col: usize) -> Coords{
        Coords {
            row,
            col
        }
    }
}

pub enum Direction {
    Right(Coords),
    Down(Coords),
    Left(Coords),
    Up(Coords)
}

impl Direction {
    pub fn siguiente(self, c: Coords) -> Direction {
        match self {
            Direction::Right(_) => Direction::Down(c),
            Direction::Down(_) => Direction::Left(c),
            Direction::Left(_) => Direction::Up(c),
            Direction::Up(_) => Direction::Right(c)
        }
    }
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![]; // Result vec

    let mut minus: usize = 0; // menos N
    let mut dir: Direction = Direction::Right(Coords::new(0, 0)); // comportamiento
    let mut steps = matrix[0].len(); // primer limite
    let mut elements: usize = 0;
    let mut bandera = -1; // auxiliar para reducir minus

    while elements < matrix[0].len() * matrix.len() {
        let mut counter: usize = 0;

        
        while counter < steps - minus {

            // logica de movimiento
            
            let aux_push = match dir {
                Direction::Right(coords) => {
                    matrix[coords.row][coords.col + counter]
                },
                Direction::Left(coords) => {
                    matrix[coords.row][coords.col - counter]
                },
                Direction::Down(coords) => {
                    matrix[coords.row + counter][coords.col]
                },
                Direction::Up(coords) => {
                    matrix[coords.row - counter][coords.col]
                }
            };

            res.push(aux_push);

            elements += 1;
            counter += 1;
        }

        
        
        dir = match dir {
            Direction::Right(c2) => {
                steps = matrix.len();
                dir.siguiente(Coords::new(c2.row + 1, c2.col + matrix[0].len() - minus - 1))
            },
            Direction::Down(c2) => {
                steps = matrix[0].len();
                dir.siguiente(Coords::new(c2.row + matrix.len() - minus - 1,c2.col - 1))
            },
            Direction::Left(c2) => {
                steps = matrix.len();
                dir.siguiente(Coords::new(c2.row - 1, c2.col - (matrix[0].len() - minus - 1)))
            },
            Direction::Up(c2) => {
                steps = matrix[0].len();
                dir.siguiente(Coords::new(c2.row - (matrix.len() - minus - 1),c2.col + 1))
            },
        };


        if minus == 0 {
            minus += 1;
        } else if bandera == 1 {
            minus += 1;
            bandera *= -1;
        } else {
            bandera *= -1;
        }
        
    }
   
    res
}

fn main() {
    
    /* 
    let matrix = vec![
        vec![1,2,3,4,5,6,7,8,9,10],
        vec![1,2,3,4,5,6,7,8,9,10],
        vec![1,2,3,4,5,6,7,8,9,10],
        vec![1,2,3,4,5,6,7,8,9,10],
        vec![1,2,3,4,5,6,7,8,9,10],
        vec![1,2,3,4,5,6,7,8,9,10],
        vec![1,2,3,4,5,6,7,8,9,10],
        vec![1,2,3,4,5,6,7,8,9,10],
        vec![1,2,3,4,5,6,7,8,9,10],
        vec![1,2,3,4,5,6,7,8,9,10]
    ]; */

    //println!("{:?}", spiral_order(matrix));


    let matrix = vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![4,5,6],
        vec![4,5,6],
        vec![4,5,6],

    ];


    println!("{:?}", spiral_order(matrix));

}


