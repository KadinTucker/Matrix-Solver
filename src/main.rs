use std::str;

fn main() {
    println!("Hello, world.");

    let m = create_matrix(vec![vec![1.0, 2.0], vec![4.0, 3.0]]);

    println!("{}", access(&m, 1, 0).unwrap());

    let v1 = vec![2.0, 3.0, 4.0];
    let v2 = vec![5.0, 2.0, 6.0];
    let v3 = vec![4.0, 3.0];

    println!("{}", vector_to_string(&add_vectors(&v1, &v2).unwrap()));

    println!("{}", matrix_to_string(&m));

}

struct Matrix {
    nrows: usize,
    ncols: usize,
    values: Vec<Vec<f64>>,
}

fn create_matrix(matrix: Vec<Vec<f64>>) -> Matrix {
    let ncols = match matrix.get(0) {
        Some(T) => T.len(),
        None => 0
    };
    Matrix {
        nrows: matrix.len(),
        ncols: ncols,
        values: matrix
    }
}

fn vector_to_string(vector: &Vec<f64>) -> String {
    let mut s: String = String::from("[ ");
    for element in vector.iter() {
        s += &(element.to_string() + " ");
    }
    s += "]";
    s
}

fn matrix_to_string(matrix: &Matrix) -> String {
    let mut s: String = String::new();
    for row in matrix.values.iter() {
        s += &(vector_to_string(&row) + "\n");
    }
    s
}

fn access(matrix: &Matrix, row: usize, col: usize) -> Option<&f64> {
    match matrix.values.get(row) {
        Some(T) => T.get(col),
        None => None
    }
}

fn scale_vector(vector: &Vec<f64>, scalar: f64) -> Vec<f64> {
    let mut scaled: Vec<f64> = Vec::new();
    for element in vector.iter() {
        scaled.push(*element * scalar);
    }
    scaled
}

fn add_vectors(vector1: &Vec<f64>, vector2: &Vec<f64>) -> Option<Vec<f64>> {
    if vector1.len() != vector2.len() {
        ()
    }
    let mut sum: Vec<f64> = Vec::new();
    for index in 0..vector1.len() {
        sum.push(vector1.get(index).unwrap() + vector2.get(index).unwrap());
    }
    Some(sum)
}