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
For the system of equations:<br>
x + y = 3<br>
2x + 3y = 5<br>
\[
\begin{bmatrix}
1 & 1 & | & 3 \\
2 & 3 & | & 5
\end{bmatrix}
\]

