use std::fmt::Debug;
use std::io::{self, Read};
use std::vec;

// --- DEFINE EPSILON ---
const EPSILON: f64 = 1e-9;

// --- SOLUTION TYPE ---
#[derive(Debug)]
enum SolutionType {
    Inconsistent,
    Infinite,
    Unique(Vec<f64>),
}

// --- FUNCTION ---
//READ MATRIX
fn read_matrix_from_input() -> Result<Vec<Vec<f64>>, String> {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return Err("FAILED TO READ STANDARD INPUT".to_string());
    }

    let matrix: Vec<Vec<f64>> = input
        .lines()
        .map(|s| s.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<f64>().ok())
                .collect::<Vec<f64>>()
        })
        .collect();

    println!();

    //MATRIX NOT EMPTY
    if matrix.is_empty() {
        return Err("NO VALID MATRIX VARIABLE".to_string());
    }
    if matrix[0].is_empty() {
        return Err("MATRIX ROW IS EMPTY".to_string());
    }

    let cols = matrix[0].len();
    if cols == 0 {
        return Err("MATRIX MUST HAVE ONE COLUMN".to_string());
    }
    if matrix.iter().any(|row| row.len() != cols) {
        return Err("INCONSISTENT NUMBER OF COLUMN IN ROW".to_string());
    }
    if cols < 1 {
        return Err("AUGMENTED MATRIX MUST HAVE AT LEAST THE CONSTANT COLUMN".to_string());
    }

    Ok(matrix)
}

//SWAP ROW
fn swap_rows(matrix: &mut Vec<Vec<f64>>, row1: usize, row2: usize) {
    if row1 < matrix.len() && row2 < matrix.len() {
        matrix.swap(row1, row2);
    }
}

//GAUSS-JORDAN-RREF
fn gauss_jordan_elimination(matrix: &mut Vec<Vec<f64>>) {
    if matrix.is_empty() || matrix[0].is_empty() {
        return;
    }
    let row_count = matrix.len();
    let col_count = matrix[0].len();
    let var_count = col_count.saturating_sub(1);

    let mut pivot_row = 0;
    let mut lead_col = 0;

    while pivot_row < row_count && lead_col < var_count {
        //1.FIND ELEMENT
        let mut i = pivot_row;
        while i < row_count && matrix[i][lead_col].abs() < EPSILON {
            i += 1;
        }

        //COLUMN NO ELEMENT -> NEXT COLUMN
        if i == row_count {
            lead_col += 1;
            continue;
        }

        //2.SWAP ROW
        if i != pivot_row {
            swap_rows(matrix, i, pivot_row);
        }

        //3.INIT ELEMENT
        let pivot_value = matrix[pivot_row][lead_col];
        if (pivot_value - 1.0).abs() > EPSILON {
            for j in lead_col..col_count {
                matrix[pivot_row][j] /= pivot_value;
                if matrix[pivot_row][j].abs() < EPSILON {
                    matrix[pivot_row][j] = 0.0;
                }
            }
        }
        matrix[pivot_row][lead_col] = 1.0;

        //4.ELIMINATE OTHER ENTRIES IN THE PIVOT COLUMN
        for k in 0..row_count {
            if k != pivot_row {
                let factor = matrix[k][lead_col];
                if factor.abs() > EPSILON {
                    //ONLY PERFORM THE OPERATION FOR NON-ZERO FACTORS
                    //matrix[k][col] = matrix[k][col] - factor * matrix[pivot_row][col]
                    for j in lead_col..col_count {
                        matrix[k][j] -= factor * matrix[pivot_row][j];
                        //CLEAN UP NEAR-ZERO VALUES
                        if matrix[k][j].abs() < EPSILON {
                            matrix[k][j] = 0.0;
                        }
                    }
                    matrix[k][lead_col] = 0.0;
                }
            }
        }

        //5.NEXT ELEMENT
        pivot_row += 1;
        lead_col += 1;
    }
}

//ANALYZE RREF SOLUTION TYPE
fn analyze_rref(matrix: &Vec<Vec<f64>>) -> SolutionType {
    if matrix.is_empty() || matrix[0].is_empty() {
        //IF IT IS A 0XN MATRIX, CONSIDER IT AS INFINITE SOLUTIONS (0 EQUATIONS, N>0 VARIABLES) OR A UNIQUE SOLUTION (0=0) (0 EQUATIONS, 0 VARIABLES)
        return SolutionType::Inconsistent;
    }
    let row_count = matrix.len();
    let col_count = matrix[0].len();
    let var_count = col_count.saturating_sub(1); // NUMBER OF VARIABLES (TOTAL COLS - 1)

    let mut rank = 0; // INITIALIZE THE RANK OF THE MATRIX
    let mut is_inconsistent = false; // FLAG TO TRACK IF THE SYSTEM IS INCONSISTENT

    for r in 0..row_count {
        let mut first_nonzero_col: Option<usize> = None;
        for c in 0..col_count {
            if matrix[r][c].abs() > EPSILON {
                first_nonzero_col = Some(c);
                break;
            }
        }

        match first_nonzero_col {
            Some(pivot_col) => {
                if pivot_col == var_count {
                    //[0 ... 0 | NON-ZERO]
                    is_inconsistent = true;
                    break;
                } else {
                    rank += 1;
                }
            }
            None => {
                //THIS ROW IS [0 ... 0 | 0]
                //MOVE TO THE NEXT ROW
                continue;
            }
        }
    }

    if is_inconsistent {
        SolutionType::Inconsistent
    } else {
        if rank < var_count {
            SolutionType::Infinite
        } else {
            // RANK = VAR COUNT
            let mut solution = vec![0.0; var_count];
            let mut temp_rank = 0;
            for r in 0..row_count {
                let mut pivot_col_in_row: Option<usize> = None;
                for c in 0..var_count {
                    if matrix[r][c].abs() > EPSILON {
                        pivot_col_in_row = Some(c);
                        break;
                    }
                }
                if let Some(pc) = pivot_col_in_row {
                    if pc < var_count {
                        solution[pc] = matrix[r][var_count];
                        temp_rank += 1;
                    }
                    if temp_rank == rank {
                        break;
                    }
                }
            }

            if temp_rank != rank {
                eprintln!("WARNING: RANK CALCULATION MISMATCH...");
                return SolutionType::Infinite;
            }

            SolutionType::Unique(solution)
        }
    }
}

//VERITY SOLUTION
fn verify_solution(original_matrix: &Vec<Vec<f64>>, solution_vector: &Vec<f64>) {
    if original_matrix.is_empty() {
        return;
    }
    let row_count = original_matrix.len();
    let col_count = original_matrix[0].len();
    let var_count = col_count.saturating_sub(1);

    if var_count != solution_vector.len() {
        println!(
            "VERIFY FAILED: SOLUTION VECTOR ({}) DOES NOT MATCH VARIABLE COUNT ({})",
            solution_vector.len(),
            var_count
        );
        return;
    }
    if var_count == 0 && row_count > 0 && original_matrix.iter().all(|row| row[0].abs() < EPSILON) {
        println!("\nVERIFY: SYSTEM HAS 0 VARIABLE, EQUATION ARE 0 = 0");
        return;
    }
    if var_count == 0 && row_count > 0 && original_matrix.iter().any(|row| row[0].abs() > EPSILON) {
        println!("\nVERIFY FAILED: SYSTEM HAS 0 VARIABLE BUT NON-ZERO CONST");
        return;
    }
    if var_count == 0 && row_count == 0 {
        println!("\nVERIFY: MATRIX IS 0x0 SYSTEM");
        return;
    }

    println!("\nVERIFY SOLUTION ORIGINAL EQUATION:");
    let mut max_diff: f64 = 0.0;
    for i in 0..row_count {
        let mut computed_lhs = 0.0;
        for j in 0..var_count {
            computed_lhs += original_matrix[i][j] * solution_vector[j];
        }
        let target_rhs = original_matrix[i][var_count];
        let diff = (computed_lhs - target_rhs).abs();
        max_diff = max_diff.max(diff);

        println!(
            "  EQ {}: LHS={:.4} vs RHS={:.4} (DIFF={:.2e})",
            i, computed_lhs, target_rhs, diff
        );
    }
    println!(
        "VERIFY FINISH. MAX ABSOLUTE DIFFERENCE: {:.2e}",
        max_diff
    );
    if max_diff > EPSILON * 10.0 {
        println!("WARING: VERIFY DIFFERENCE SEEMS LARGE");
    }
}

//PRINT MATRIX
fn print_matrix(matrix: &Vec<Vec<f64>>) {
    if matrix.is_empty() {
        println!("[]");
        return;
    }
    println!("[");
    for row in matrix {
        print!("  [");
        for (i, val) in row.iter().enumerate() {
            print!("{:.4}", val);
            if i < row.len() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }
    println!("]");
}

// --- MAIN ---
fn main() {
    //READ MATRIX
    let mut matrix = match read_matrix_from_input() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("ERROR READ INPUT: {}", e);
            return;
        }
    };

    if matrix.is_empty() || matrix[0].is_empty() {
        println!("MATRIX IS EMPTY");
        return;
    }

    //CLONE MATRIX
    let original_matrix = matrix.clone();

    println!("ORIGINAL MATRIX:");
    print_matrix(&original_matrix);
    println!();

    gauss_jordan_elimination(&mut matrix);

    println!("RREF:");
    print_matrix(&matrix);
    println!();

    //ANSWER
    let solution_type = analyze_rref(&matrix);

    match solution_type {
        SolutionType::Inconsistent => {
            println!("RESULT: INCONSISTENT (NO SOLUTION)");
        }
        SolutionType::Infinite => {
            println!("RESULT: SOLUTION SET (INFINITELY MANY SOLUTION)");
        }
        SolutionType::Unique(solution_vec) => {
            println!("RESULT: UNIQUE SOLUTION");
            if solution_vec.is_empty() {
                println!("RESULT: UNIQUE SOLUTION: 0 (NOTE: SYSTEM HAS 0 VARIABLE 0 = 0)");
            } else {
                for (i, val) in solution_vec.iter().enumerate() {
                    println!("  SOLUTION VAR_{} = {:.4}", i, val);
                }
                //VERIFY SOLUTION
                verify_solution(&original_matrix, &solution_vec);
            }
        }
    }
}
