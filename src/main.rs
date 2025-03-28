use std::io::{self, Read};

fn read_matrix_from_input() -> Vec<Vec<f64>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect()
}

fn validate_matrix_structure(matrix: &mut Vec<Vec<f64>>) -> Result<(), String> {
    if matrix.len() + 1 != matrix[0].len() {
        return Err("INCONSISTENT".to_string());
    }
    Ok(())
}

fn normalize_row(matrix: &mut Vec<Vec<f64>>, row: usize) -> Result<(), String> {
    if matrix[row][row] == 0.0 {
        return Err("matrix[row][row] = 0".to_string());
    }

    let pivot = matrix[row][row];
    for i in 0..matrix[row].len() {
        matrix[row][i] /= pivot;
        if matrix[row][i] == -0.0 {
            matrix[row][i] = 0.0;
        }
    }
    Ok(())
}

fn swap_rows(matrix: &mut Vec<Vec<f64>>, row1: usize, row2: usize) -> Result<(), String> {
    if row1 >= matrix.len() || row2 >= matrix.len() {
        return Err("Index out of matrix bounds".to_string());
    }
    matrix.swap(row1, row2);
    Ok(())
}

fn subtract_rows(matrix: &mut Vec<Vec<f64>>, target_row: usize, ref_row: usize, factor: f64) {
    for i in 0..matrix[target_row].len() {
        matrix[target_row][i] -= matrix[ref_row][i] * factor;
        if matrix[target_row][i] == -0.0 {
            matrix[target_row][i] = 0.0;
        }
    }
}

fn perform_gaussian_elimination(matrix: &mut Vec<Vec<f64>>) -> Result<(), String> {
    let row_count = matrix.len();
    let mut swap_attempts = 0;
    let mut i = 0;

    while i < row_count {
        match normalize_row(matrix, i) {
            Ok(_) => {
                println!("Iteration {}: Normalized {:?}", i, matrix);
                swap_attempts = 0;
            }
            Err(e) => {
                println!("Iteration {}: Error: {}", i, e);
                match swap_rows(matrix, i, i + 1 + swap_attempts) {
                    Ok(_) => {
                        println!("Iteration {}: Swapped rows {:?}", i, matrix);
                        swap_attempts += 1;
                        continue;
                    }
                    Err(e) => {
                        println!("Iteration {}: {:?}", i, e);
                        return Err(e);
                    }
                }
            }
        }

        for j in 0..row_count {
            if j != i {
                let factor = matrix[j][i];
                subtract_rows(matrix, j, i, factor);
            }
        }
        println!("Iteration {}: Row operations {:?}", i, matrix);
        i += 1;
    }
    Ok(())
}

fn verify_solution(original_matrix: &Vec<Vec<f64>>, solution_matrix: &Vec<Vec<f64>>) {
    for i in 0..original_matrix.len() {
        let mut computed_value = 0.0;
        for j in 0..original_matrix.len() {
            computed_value += original_matrix[i][j] * solution_matrix[j][solution_matrix.len()];
        }
        println!("{}", computed_value);
    }
}

fn main() {
    let mut matrix = read_matrix_from_input();
    let original_matrix = matrix.clone();
    println!();

    match validate_matrix_structure(&mut matrix) {
        Ok(_) => {
            match perform_gaussian_elimination(&mut matrix) {
                Ok(_) => {
                    for i in 0..matrix.len() {
                        println!("SOLUTION {} = {}", i, matrix[i][matrix[i].len() - 1]);
                    }
                    verify_solution(&original_matrix, &matrix);
                }
                Err(e) => {
                    println!("{}", e);
                    let mut zero_count = 0;
                    for i in 0..matrix[matrix.len() - 1].len() - 1 {
                        if matrix[matrix.len() - 1][i] == 0.0 {
                            zero_count += 1;
                        }
                    }
                    if zero_count == matrix[matrix.len() - 1].len() - 1
                        && matrix[matrix.len() - 1][matrix[0].len() - 1] != 0.0
                    {
                        println!("INCONSISTENT");
                    } else {
                        println!("SOLUTION SET");
                    }
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
