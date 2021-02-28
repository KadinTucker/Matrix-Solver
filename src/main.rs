use std::str;

fn main() {
    println!("Hello, world.");

    let mut m = create_matrix(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 3, 3).unwrap();

    // println!("{}", access(&m, 1, 0).unwrap());

    // let v1 = vec![2.0, 3.0, 4.0];
    // let v2 = vec![5.0, 2.0, 6.0];
    // let v3 = vec![4.0, 3.0];

    // println!("{}", vector_to_string(&add_vectors(&v1, &v2).unwrap()));

    // println!("{}", matrix_to_string(&m));

    print_matrix(&m);

    println!("");

    m = eliminate(m, 0, 1, 0);

    print_matrix(&m);

}

struct Matrix {
    nrows: usize,
    ncols: usize,
    values: Vec<f64>,
}

fn create_matrix(values: Vec<f64>, nrows: usize, ncols: usize) -> Option<Matrix> {
    let elts = values.len();
    match nrows * ncols {
        elts => Some(Matrix {
            nrows: nrows,
            ncols: ncols,
            values: values
        }),
        _ => None
    }
}

fn print_matrix(matrix: &Matrix) {
    for i in 0..matrix.nrows {
        print!("[ ");
        for j in 0..matrix.ncols {
            print!("{} ", matrix.values[i * matrix.nrows + j]);
        }
        println!("]");
    }
}

fn scale_row(mut matrix: Matrix, row: usize, scalar: f64) -> Matrix {
    if row < matrix.nrows {
        for i in (row * matrix.ncols)..(row * matrix.ncols + matrix.ncols) {
            matrix.values[i] *= scalar;
        }
    }
    matrix
}

fn add_and_scale(mut matrix: Matrix, origin_row: usize, dest_row: usize, scalar: f64) -> Matrix {
    if origin_row < matrix.nrows && dest_row < matrix.nrows {
        for i in 0..matrix.ncols {
            matrix.values[i + dest_row * matrix.ncols] += matrix.values[i + origin_row * matrix.ncols] * scalar;
        }
    }
    matrix
}

fn add_rows(mut matrix: Matrix, origin_row: usize, dest_row: usize) -> Matrix {
    add_and_scale(matrix, origin_row, dest_row, 1.0)
}

fn eliminate(mut matrix: Matrix, origin_row: usize, dest_row: usize, dest_col: usize) -> Matrix {
    let elim_scalar = -matrix.values[matrix.ncols * dest_row + dest_col] / matrix.values[matrix.ncols * origin_row + dest_col];
    add_and_scale(matrix, origin_row, dest_row, elim_scalar)
}