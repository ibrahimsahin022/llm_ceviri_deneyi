use std::io::Read;

const MAX_N: usize = 10;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).ok();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    if n == 0 || n > MAX_N {
        return;
    }

    // Create augmented matrix [A|I]
    let mut a: Vec<Vec<f64>> = vec![vec![0.0; 2 * n]; n];

    // Read matrix A
    for i in 0..n {
        for j in 0..n {
            let val: f64 = tokens.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
            a[i][j] = val;
        }
    }

    // Initialize right half to identity matrix
    for i in 0..n {
        for j in 0..n {
            a[i][n + j] = if i == j { 1.0 } else { 0.0 };
        }
    }

    // Gauss-Jordan elimination
    for col in 0..n {
        let pivot = a[col][col];
        if pivot.abs() < 1e-9 {
            println!("SINGULAR");
            return;
        }

        // Divide pivot row by pivot
        for j in 0..(2 * n) {
            a[col][j] /= pivot;
        }

        // Eliminate column
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = a[row][col];
            for j in 0..(2 * n) {
                a[row][j] -= factor * a[col][j];
            }
        }
    }

    // Output the inverse (right half of augmented matrix)
    for i in 0..n {
        for j in 0..n {
            print!("{:.4} ", a[i][n + j]);
        }
        println!();
    }
}
