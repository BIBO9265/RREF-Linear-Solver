# RREF-Linear-Solver
A program implementing the Gauss-Jordan Elimination method to solve systems of linear equations. It reduces the augmented matrix to Reduced Row Echelon Form (RREF), providing a unique solution, infinite solutions, or identifying inconsistency when no solution exists.
## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/BIBO9265/RREF-Linear-Solver.git
   ```

2. Go to the project directory:
   ```bash
   cd RREF-Linear-Solver
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage
To run the program, simply provide the augmented matrix as input:
```bash
cargo run
```
## Example 1: Unique Solution
For the system of equations:
```bash
x + y = 3
2x + 3y = 5
```

The augmented matrix would be represented as:
```bash
1 1 3
2 3 5
```

Input:
```bash
1 1 3
2 3 5
```

Output:
```bash
ORIGINAL MATRIX:
[
  [1.0000, 1.0000, 3.0000]
  [2.0000, 3.0000, 5.0000]
]

RREF:
[
  [1.0000, 0.0000, 4.0000]
  [0.0000, 1.0000, -1.0000]
]

RESULT: UNIQUE SOLUTION
  SOLUTION VAR_0 = 4.0000
  SOLUTION VAR_1 = -1.0000

VERIFY SOLUTION ORIGINAL EQUATION:
  EQ 0: LHS=3.0000 vs RHS=3.0000 (DIFF=0.00e0)
  EQ 1: LHS=5.0000 vs RHS=5.0000 (DIFF=0.00e0)
VERIFY FINISH. MAX ABSOLUTE DIFFERENCE: 0.00e0
```

This will result in a unique solution:
```bash
x = 4
y = -1
```

Verification:<br>
```bash
Substitute 𝑥 = 4 and 𝑦 = − 1 into the original equations:
First equation:
4 + ( − 1 ) = 3
This is true,as 3 = 3.
```
## Example 2: Solutions SET (INFINITELY MANY SOLUTION)
For the system of equations:
```bash
x + y = 3
2x + 2y = 6
```

The augmented matrix would be represented as:
```bash
1 1 3
2 2 6
```

Input:
```bash
1 1 3
2 2 6
```

Output:
```bash
ORIGINAL MATRIX:
[
  [1.0000, 1.0000, 3.0000]
  [2.0000, 2.0000, 6.0000]
]

RREF:
[
  [1.0000, 1.0000, 3.0000]
  [0.0000, 0.0000, 0.0000]
]

RESULT: SOLUTION SET (INFINITELY MANY SOLUTION)
```

This system has infinite solutions, as there are dependent equations, and one equation can be derived from the other.

## Example 3: INCONSISTENT (NO SOLUTION)
For the system of equations:
```bash
x + y = 3
x + y = 5
```

The augmented matrix would be represented as:
```bash
1 1 3
1 1 5
```

Input:
```bash
1 1 3
1 1 5
```

Output:
```bash
ORIGINAL MATRIX:
[
  [1.0000, 1.0000, 3.0000]
  [1.0000, 1.0000, 5.0000]
]

RREF:
[
  [1.0000, 1.0000, 3.0000]
  [0.0000, 0.0000, 2.0000]
]

RESULT: INCONSISTENT (NO SOLUTION)
```

This system is inconsistent and has no solution because the two equations contradict each other.

## Latest Updates

This update focuses on enhancing the solver's stability and precision, optimizing the core algorithm, and improving the clarity of the output.

### Key Changes:

* **Improved Precision and Numerical Stability:**
  To handle potential minor errors in floating-point computations more reliably, a small tolerance value `const EPSILON: f64 = 1e-9;` (i.e., $10^{-9}$) has been introduced. This constant is used when comparing floating-point numbers, making checks for zero or equality more robust and leading to more accurate solutions.
  *Self-learning point: Using an epsilon value is a common practice in numerical programming when dealing with floating-point comparisons, as direct equality checks (`==`) can be unreliable due to how these numbers are stored.*

* **Optimized Core Reduction Algorithm:**
  The core algorithm for reducing matrices to Reduced Row Echelon Form (RREF), based on Gaussian-Jordan elimination, has been updated and optimized. This improves both the efficiency and accuracy of the reduction process.

* **Enhanced Logic for Solution Determination:**
  The solver now more clearly and accurately determines the nature of the linear system's solution based on the RREF result. It can precisely identify:
    * **Unique Solutions:** When every variable corresponds to a pivot.
    * **Infinite Solutions:** When free variables exist. The solver will help indicate which variables are free.
    * **No Solution:** When an inconsistent row (like `0 = 1`) appears in the RREF.

* **More Systematic and User-Friendly Output:**
  The format for displaying the solver's findings regarding the solution(s) has been made more systematic and easier to interpret. The output now clearly indicates whether a unique solution, infinite solutions, or no solution was found, and presents the results in a more structured way for better readability.








