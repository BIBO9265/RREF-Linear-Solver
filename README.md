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
Iteration 0: Normalized [[1.0, 1.0, 3.0], [2.0, 3.0, 5.0]]
Iteration 0: Row operations [[1.0, 1.0, 3.0], [0.0, 1.0, -1.0]]
Iteration 1: Normalized [[1.0, 1.0, 3.0], [0.0, 1.0, -1.0]]
Iteration 1: Row operations [[1.0, 0.0, 4.0], [0.0, 1.0, -1.0]]
SOLUTION 0 = 4
SOLUTION 1 = -1
3
5
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
## Example 2: Infinite Solutions
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
Iteration 0: Normalized [[1.0, 1.0, 3.0], [2.0, 2.0, 6.0]]
Iteration 0: Row operations [[1.0, 1.0, 3.0], [0.0, 0.0, 0.0]]
Iteration 1: Error: matrix[row][row] = 0
Iteration 1: "Index out of matrix bounds"
Index out of matrix bounds
SOLUTION SET
```

This system has infinite solutions, as there are dependent equations, and one equation can be derived from the other.

## Example 3: No Solution (Inconsistent)
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
Iteration 0: Normalized [[1.0, 1.0, 3.0], [1.0, 1.0, 5.0]]
Iteration 0: Row operations [[1.0, 1.0, 3.0], [0.0, 0.0, 2.0]]
Iteration 1: Error: matrix[row][row] = 0
Iteration 1: "Index out of matrix bounds"
Index out of matrix bounds
INCONSISTENT
```

This system is inconsistent and has no solution because the two equations contradict each other.












