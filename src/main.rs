use make_24::calculation;
use make_24::calculation::{Calculation, Operator};
use std::io::{self, Write};
use std::thread::panicking;

#[derive(Debug)]
enum ParseInputError {
    InvalidInput,
    IoError(io::Error),
}

impl From<io::Error> for ParseInputError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

fn main() -> Result<(), ParseInputError> {
    let digits = get_input()?;
    let calcs = Calculation::new_calcs(digits, None);

    let mut res = Vec::new();
    for calc in calcs {
        res.append(calculation::collapse(calc).as_mut());
    }

    for calculation in res {
        calculation.explain();
    }

    return Ok(());
}

fn get_input() -> Result<Vec<i32>, ParseInputError> {
    print!("Enter your numbers XXXX: ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let digits: Vec<i32> = buffer
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .map(|ch| ch.to_digit(10).unwrap() as i32)
        .collect();

    if digits.len() != buffer.len() - 1 {
        panic!("Cannot");
    }

    Ok(digits)
}
