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
'''



