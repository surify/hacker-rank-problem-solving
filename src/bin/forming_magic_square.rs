/* https://www.hackerrank.com/challenges/magic-square-forming/problem?isFullScreen=true */
fn main() {
    let mat = vec![
        vec![4, 9, 2],
        vec![3, 5, 7],
        vec![8, 1, 5],
    ];

    let min_steps = forming_magic_square(&mat);

    println!("Min steps: {min_steps}");
}
fn forming_magic_square(input_matrix: &[Vec<i32>]) -> i32 {
    let mut min_steps = i32::MAX;
    let magic_square = vec![
        vec![2, 9, 4],
        vec![7, 5, 3],
        vec![6, 1, 8],
    ];

    let rr = matrix_row_reverse(&magic_square);
    min_steps = min(min_steps, matrix_absolute_difference(input_matrix, &rr));

    let rrt = matrix_transpose(&rr);
    min_steps = min(min_steps, matrix_absolute_difference(input_matrix, &rrt));

    let cr = matrix_col_reverse(&magic_square);
    min_steps = min(min_steps, matrix_absolute_difference(input_matrix, &cr));

    let crt = matrix_transpose(&cr);
    min_steps = min(min_steps, matrix_absolute_difference(input_matrix, &crt));

    let t = matrix_transpose(&magic_square);
    min_steps = min(min_steps, matrix_absolute_difference(input_matrix, &t));

    let rrtrr = matrix_row_reverse(&rrt);
    min_steps = min(min_steps, matrix_absolute_difference(input_matrix, &rrtrr));

    let rrtrrt = matrix_transpose(&rrtrr);
    min_steps = min(min_steps, matrix_absolute_difference(input_matrix, &rrtrrt));

    min_steps
}

fn print_matrix(mat: &[Vec<i32>]) {
    for row in mat {
        for cell in row {
            print!("{} ", *cell);
        }
        println!();
    }
}

fn min(a: i32, b: i32) -> i32 {
    // println!("Min() | a: {a} | b: {b}");
    if a < b {
        return a;
    }
    b
}

/* absolute difference between two square 3x3 matrices */
fn matrix_absolute_difference(mat1: &[Vec<i32>], mat2: &[Vec<i32>]) -> i32 {
    // print_matrix(mat2);
    let mut difference = 0;

    for i in 0..3 {
        for j in 0..3 {
            difference += (mat1[i][j] - mat2[i][j]).abs();
        }
    }

    difference
}

/* exchanges first and last rows of a 3x3 square matrix */
fn matrix_row_reverse(mat: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut reversed: Vec<Vec<i32>> = vec![
        vec![0; 3],
        vec![0; 3],
        vec![0; 3],
    ];

    for i in 0..3 {
        reversed[0][i] = mat[2][i];
        reversed[2][i] = mat[0][i];
        reversed[1][i] = mat[1][i];
    }

    reversed
}

/* exchanges first and last columns of a 3x3 square matrix */
fn matrix_col_reverse(mat: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut reversed: Vec<Vec<i32>> = vec![
        vec![0; 3],
        vec![0; 3],
        vec![0; 3],
    ];

    for i in 0..3 {
        reversed[i][0] = mat[i][2];
        reversed[i][2] = mat[i][0];
        reversed[i][1] = mat[i][1];
    }

    reversed
}

/* 3x3 matrix transpose*/
fn matrix_transpose(mat: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut transposed: Vec<Vec<i32>> = vec![
        vec![0; 3],
        vec![0; 3],
        vec![0; 3],
    ];

    for i in 0..3 {
        for j in 0..3 {
            transposed[i][j] = mat[j][i];
        }
    }

    transposed
}
