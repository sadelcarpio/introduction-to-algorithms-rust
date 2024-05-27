use std::vec::Vec;

struct MatMul;

impl MatMul {
    fn naive_multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = a.len();
        let mut c = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    c[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        c
    }

    fn recursive_multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = a.len();
        let mut c = vec![vec![0; n]; n];
        MatMul::_recursive_multiply(a, b, &mut c, 0, 0, 0, 0, n);
        c
    }

    fn _recursive_multiply(
        a: &Vec<Vec<i32>>,
        b: &Vec<Vec<i32>>,
        c: &mut Vec<Vec<i32>>,
        a_col: usize, a_row: usize, b_col: usize, b_row: usize, n: usize,
    ) {
        if n == 1 {
            c[a_row][b_col] += a[a_row][a_col] * b[b_row][b_col];
            return;
        }
        let new_n = n / 2;
        MatMul::_recursive_multiply(a, b, c, a_col, a_row, b_col, b_row, new_n);
        MatMul::_recursive_multiply(a, b, c, a_col + new_n, a_row, b_col, b_row + new_n, new_n);
        MatMul::_recursive_multiply(a, b, c, a_col, a_row, b_col + new_n, b_row, new_n);
        MatMul::_recursive_multiply(a, b, c, a_col + new_n, a_row, b_col + new_n, b_row + new_n, new_n);
        MatMul::_recursive_multiply(a, b, c, a_col, a_row + new_n, b_col, b_row, new_n);
        MatMul::_recursive_multiply(a, b, c, a_col + new_n, a_row + new_n, b_col, b_row + new_n, new_n);
        MatMul::_recursive_multiply(a, b, c, a_col, a_row + new_n, b_col + new_n, b_row, new_n);
        MatMul::_recursive_multiply(a, b, c, a_col + new_n, a_row + new_n, b_col + new_n, b_row + new_n, new_n);
    }

    fn strassen_multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = a.len();
        MatMul::_strassen_multiply(a, b, 0, 0, 0, 0, n)
    }

    fn _strassen_multiply(
        a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>,
        a_col: usize, a_row: usize, b_col: usize, b_row: usize, n: usize) -> Vec<Vec<i32>> {
        let mut c = vec![vec![0; n]; n];
        if n == 1 {
            return vec![vec![a[a_row][a_col] * b[b_row][b_col]]];
        }
        let new_n = n / 2;
        let mut s = vec![vec![0; 5 * new_n]; 2 * new_n];
        let mut p = vec![vec![vec![0; n]; n]; 7];

        MatMul::matrix_sum(b, b, &mut s, b_col + new_n, b_row, b_col + new_n, b_row + new_n, 0, 0, new_n, true);  // b12 - b22
        MatMul::matrix_sum(a, a, &mut s, a_col, a_row, a_col + new_n, a_row, new_n, 0, new_n, false);  // a11 + a12
        MatMul::matrix_sum(a, a, &mut s, a_col, a_row + new_n, a_col + new_n, a_row + new_n, 2 * new_n, 0, new_n, false);  // a21 + a22
        MatMul::matrix_sum(b, b, &mut s, b_col, b_row + new_n, b_col, b_row, 3 * new_n, 0, new_n, true);  // b21 - b11
        MatMul::matrix_sum(a, a, &mut s, a_col, a_row, a_col + new_n, a_row + new_n, 4 * new_n, 0, new_n, false);  // a11 + a22
        MatMul::matrix_sum(b, b, &mut s, b_col, b_row, b_col + new_n, b_row + new_n, 0, new_n, new_n, false);  // b11 + b22
        MatMul::matrix_sum(a, a, &mut s, a_col + new_n, a_row, a_col + new_n, a_row + new_n, new_n, new_n, new_n, true);  // a12 - a22
        MatMul::matrix_sum(b, b, &mut s, b_col, b_row + new_n, b_col + new_n, b_row + new_n, 2 * new_n, new_n, new_n, false);  // b21 + b22
        MatMul::matrix_sum(a, a, &mut s, a_col, a_row, a_col, a_row + new_n, 3 * new_n, new_n, new_n, true);  // a11 - a21
        MatMul::matrix_sum(b, b, &mut s, b_col, b_row, b_col + new_n, b_row, 4 * new_n, new_n, new_n, false);  // b11 + b12

        p[0] = MatMul::_strassen_multiply(a, &s, a_col, a_row, 0, 0, new_n);
        p[1] = MatMul::_strassen_multiply(&s, b, new_n, 0, b_col + new_n, b_row + new_n, new_n);
        p[2] = MatMul::_strassen_multiply(&s, b, 2 * new_n, 0, b_col, b_row, new_n);
        p[3] = MatMul::_strassen_multiply(a, &s, a_col + new_n, a_row + new_n, 3 * new_n, 0, new_n);
        p[4] = MatMul::_strassen_multiply(&s, &s, 4 * new_n, 0, 0, new_n, new_n);
        p[5] = MatMul::_strassen_multiply(&s, &s, new_n, new_n, 2 * new_n, new_n, new_n);
        p[6] = MatMul::_strassen_multiply(&s, &s, 3 * new_n, new_n, 4 * new_n, new_n, new_n);

        MatMul::c11(&p, &mut c, new_n);
        MatMul::c12(&p, &mut c, new_n);
        MatMul::c21(&p, &mut c, new_n);
        MatMul::c22(&p, &mut c, new_n);

        c
    }

    fn c11(p: &Vec<Vec<Vec<i32>>>, c: &mut Vec<Vec<i32>>, n: usize) {
        for i in 0..n {
            for j in 0..n {
                c[i][j] += p[4][i][j] + p[3][i][j] - p[1][i][j] + p[5][i][j];
            }
        }
    }

    fn c12(p: &Vec<Vec<Vec<i32>>>, c: &mut Vec<Vec<i32>>, n: usize) {
        for i in 0..n {
            for j in 0..n {
                c[i][j + n] += p[0][i][j] + p[1][i][j];
            }
        }
    }

    fn c21(p: &Vec<Vec<Vec<i32>>>, c: &mut Vec<Vec<i32>>, n: usize) {
        for i in 0..n {
            for j in 0..n {
                c[i + n][j] += p[2][i][j] + p[3][i][j];
            }
        }
    }

    fn c22(p: &Vec<Vec<Vec<i32>>>, c: &mut Vec<Vec<i32>>, n: usize) {
        for i in 0..n {
            for j in 0..n {
                c[i + n][j + n] += p[4][i][j] + p[0][i][j] - p[2][i][j] - p[6][i][j];
            }
        }
    }

    fn matrix_sum(
        a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>, s: &mut Vec<Vec<i32>>,
        a_col: usize, a_row: usize, b_col: usize, b_row: usize,
        s_col: usize, s_row: usize, n: usize, neg: bool,
    ) {
        for i in 0..n {
            for j in 0..n {
                s[s_row + i][s_col + j] = a[a_row + i][a_col + j] + if neg { -b[b_row + i][b_col + j] } else { b[b_row + i][b_col + j] };
            }
        }
    }
}

fn main() {
    let a = vec![vec![1, 1], vec![1, 1]];
    let b = vec![vec![2, 2], vec![2, 2]];
    let c = MatMul::strassen_multiply(&a, &b);
    for row in c.iter() {
        for val in row.iter() {
            print!(" {} ", val);
        }
        println!();
    }
}
