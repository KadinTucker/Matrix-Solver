use std::str;

/* 
TODOs:
 - Add a function that finds the inverse matrix by using Gauss-Jordan elimination
 - Implement a quick way to generate a Vandermonde matrix system given a set of data
   - perfect polynomial fitting algorithm
*/

fn main() {
    println!("Hello, world.");

    let mut m = create_matrix(vec![1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 7.0, 8.0, 8.0], 3, 3).unwrap();

    // println!("{}", access(&m, 1, 0).unwrap());

    // let v1 = vec![2.0, 3.0, 4.0];
    // let v2 = vec![5.0, 2.0, 6.0];
    // let v3 = vec![4.0, 3.0];

    // println!("{}", vector_to_string(&add_vectors(&v1, &v2).unwrap()));

    // println!("{}", matrix_to_string(&m));

    print_matrix(&m);

    println!("");

    m = reduce(m);

    print_matrix(&m);
    println!("");

    let mut sys = create_matrix(vec![-8.0, 4.0, -2.0, 1.0, 8.0, -1.0, 1.0, -1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 1.0, 1.0, 8.0, 4.0, 2.0, 1.0, 7.0], 4, 5).unwrap();

    print_matrix(&sys);
    println!("");
    sys = reduce(sys);
    print_matrix(&sys);

}

/**
 * A matrix structure of f64 values, consisting of a linear array
 * Values are accessible by row and column using below functions
 * TODO: Make the below functions 'safe', so that they fail when attempting an impossible action
 */
struct Matrix {
    nrows: usize,
    ncols: usize,
    values: Vec<f64>,
}

/**
 * Constructs a matrix given a linear array of values and the number of rows, cols. 
 * TODO: improve the safety of this function
 */
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

/**
 * Given a matrix, return the value at the given row, col
 */
fn get_at_row_col(matrix: &Matrix, row: usize, col: usize) -> f64 {
    matrix.values[row * matrix.ncols + col]
}


/**
 * Prints a matrix to the console so that it can be visualized by the programmer
 */
fn print_matrix(matrix: &Matrix) {
    for i in 0..matrix.nrows {
        print!("[ ");
        for j in 0..matrix.ncols {
            print!("{} ", matrix.values[i * matrix.ncols + j]);
        }
        println!("]");
    }
}

/**
 * Given a matrix and a row of the matrix, multiply all elements of that row by a given f64 scalar value
 */
fn scale_row(mut matrix: Matrix, row: usize, scalar: f64) -> Matrix {
    if row < matrix.nrows {
        for i in (row * matrix.ncols)..(row * matrix.ncols + matrix.ncols) {
            matrix.values[i] *= scalar;
        }
    }
    matrix
}

/**
 * Given a matrix, mofying it by adding an origin row, scaled by a scalar f64 value, 
 *  to a destination row in the matrix
 * This is a common matrix operation and will be used in the Gauss-Jordan algorithm
 */
fn add_and_scale(mut matrix: Matrix, origin_row: usize, dest_row: usize, scalar: f64) -> Matrix {
    if origin_row < matrix.nrows && dest_row < matrix.nrows {
        for i in 0..matrix.ncols {
            matrix.values[i + dest_row * matrix.ncols] += matrix.values[i + origin_row * matrix.ncols] * scalar;
        }
    }
    matrix
}

/**
 * Given a matrix, modify it by adding one origin row to a destination row
 * Uses the add_and_scale method but uses the common argument of 1.0 for the scalar
 */
fn add_rows(mut matrix: Matrix, origin_row: usize, dest_row: usize) -> Matrix {
    add_and_scale(matrix, origin_row, dest_row, 1.0)
}

/**
 * Given a matrix, an origin row, and a destination row and column, add and scale such that
 * the destination location becomes 0
 * TODO: Make this account for possible division by zero
 */
fn eliminate(mut matrix: Matrix, origin_row: usize, dest_row: usize, dest_col: usize) -> Matrix {
    let elim_scalar = -matrix.values[matrix.ncols * dest_row + dest_col] / matrix.values[matrix.ncols * origin_row + dest_col];
    add_and_scale(matrix, origin_row, dest_row, elim_scalar)
}

/**
 * Reduces the matrix to the form of [I | R], I a square identity matrix
 * Only works if there are more columns than rows
 * TODO: make it transpose the matrix in the case of more rows than columns
 */
fn reduce(mut matrix: Matrix) -> Matrix {
    // 
    let r = matrix.nrows;
    // First pass: lower triangle
    for j in 0..r {
        for i in (j+1)..r {
            matrix = eliminate(matrix, j, i, j);
        }
    }
    // Recto-verso; upper triangle
    for j in 0..r {
        println!("{}", j);
        for i in (j+1)..r {
            println!("{}", i);
            matrix = eliminate(matrix, r - j - 1, r - i - 1, r - j - 1);
        }
    }
    //Lastly, scale rows appropriately
    for i in 0..r {
        let scalar = 1.0 / get_at_row_col(&matrix, i, i);
        matrix = scale_row(matrix, i, scalar);
    }
    matrix
}