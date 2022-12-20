#[derive(Debug)]
struct Sudoku {
    board: [[u8; 9]; 9],
}

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
                        digits[num as usize - 1] = true;
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_sudoku() {
        let sudoku: Sudoku = "534678912\n\
             672195348\n\
             198342567\n\
             859761423\n\
             426853791\n\
             713924856\n\
             961537284\n\
             287419635\n\
             345286177"
            .parse()
            .unwrap();
        println!("{:?}", sudoku);
        assert!(sudoku.valid());
    }
}

