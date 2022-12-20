# Rust-Assignment

To implement the `Sudoku` struct and the `validate_sudoku` function, we will need to first parse the input string into a 2D array representing the sudoku board.
We can start by defining the `Sudoku` struct as follows:

```rust
struct Sudoku {
    board: [[u8; 9]; 9],
}

```
This will define a `Sudoku` struct with a field `board` that is a 2D array of 9x9 elements, where each element is a single digit number represented as a `u8`.

Next, we can implement a method parse on the `Sudoku` struct that will parse the input string into a `Sudoku` struct.

```rust
impl Sudoku {
    fn parse(input: &str) -> Result<Sudoku, &'static str> {
        let mut board = [[0; 9]; 9];
        let mut row = 0;
        for line in input.lines() {
            if line.len() != 9 {
                return Err("Each line must have 9 characters");
            }
            for (col, c) in line.chars().enumerate() {
                let num = c.to_digit(10).ok_or("Invalid character")?;
                board[row][col] = num as u8;
            }
            row += 1;
        }
        if row != 9 {
            return Err("There must be 9 lines in the input");
        }
        Ok(Sudoku { board })
    }
}

```

This implementation will parse the input string line by line, and for each line it will check that it has 9 characters. It will then parse each character as a digit, and assign it to the corresponding position in the `board` field.

Finally, we can implement the `valid` method that will validate the sudoku board.

```rust
impl Sudoku {
    fn valid(&self) -> bool {
        // Check that each row has unique digits
        for row in 0..9 {
            let mut digits = [false; 9];
            for col in 0..9 {
                let num = self.board[row][col];
                if digits[num as usize - 1] {
                    return false;
                }
                digits[num as usize - 1] = true;
            }
        }

        // Check that each column has unique digits
        for col in 0..9 {
            let mut digits = [false; 9];
            for row in 0..9 {
                let num = self.board[row][col];
                if digits[num as usize - 1] {
                    return false;
                }
                digits[num as usize - 1] = true;
            }
        }

        // Check that each 3x3 subgrid has unique digits
        for i in (0..9).step_by(3) {
            for j in (0..9).step_by(3) {
                let mut digits = [false; 9];
                for row in i..i+3 {
                    for col in j..j+3 {
                        let num = self.board[row][col];
                        if digits[num as usize - 1] {
                            return false;
                        }
                        digits[num as usize - 1] = true
```

To test the code, you can ```run cargo``` test in your terminal. This will execute the test case `test_validate_sudoku`, which will parse the input string into a `Sudoku` struct and then check that the sudoku board is valid.

If the test passes, you should see the following output:


![result](https://user-images.githubusercontent.com/113277972/208673943-d642807f-2aee-4aa0-a0ca-c65a7bf554b8.jpg)









