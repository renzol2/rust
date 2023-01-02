use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl Operation {
    pub fn from_char(symbol: char) -> Option<Operation> {
        match symbol {
            '+' => Some(Operation::Add),
            '-' => Some(Operation::Subtract),
            'x' | '*' => Some(Operation::Multiply),
            '/' => Some(Operation::Divide),
            '%' => Some(Operation::Modulo),
            _ => None,
        }
    }
}

// [HELPER FUNCTION - DO NOT EDIT]
pub fn get_equation_tuple(line: &String) -> (Option<&str>, Option<&str>) {
    let v: Vec<&str> = line.split(&['-', '+', 'x', '*', '/', '%'][..]).collect();

    (v.get(0).cloned(), v.get(1).cloned())
}

fn trim_parse(s: &str) -> Result<f32, ()> {
    match s.trim().parse::<f32>() {
        Ok(f) => Ok(f),
        Err(_) => Err(()),
    }
}

fn get_operation(line: &String) -> Option<Operation> {
    for c in line.chars() {
        match Operation::from_char(c) {
            Some(o) => return Some(o),
            None => (),
        }
    }
    None
}

pub fn parse_equation(line: &String) -> Result<(f32, f32, Operation), ()> {
    // Receive both numbers as strings
    let (a, b) = get_equation_tuple(line);
    let (a, b) = match (a, b) {
        (Some(a), Some(b)) => (a, b),
        _ => return Err(()),
    };

    // Get operation
    let operation = match get_operation(line) {
        Some(o) => o,
        None => return Err(()),
    };

    // Trim and parse floats
    match (trim_parse(a), trim_parse(b)) {
        (Ok(a), Ok(b)) => Ok((a, b, operation)),
        _ => Err(()),
    }
}

fn solve_equation(a: f32, b: f32, op: Operation) -> f32 {
    match op {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply => a * b,
        Operation::Divide => a / b,
        Operation::Modulo => a % b,
    }
}

pub fn solve(equation: &String) -> Result<f32, ()> {
    match parse_equation(equation) {
        Ok((a, b, op)) => Ok(solve_equation(a, b, op)),
        Err(_) => Err(()),
    }
}

pub fn solve_all(file_path: &str) -> Result<f32, ()> {
    // Read file
    let contents = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(_) => return Err(()),
    };

    // Solve equations line by line, adding results to sum
    let sum = contents
        .lines()
        .into_iter()
        .map(|l| match solve(&l.to_string()) {
            Ok(f) => f,
            Err(_) => 0.0,
        })
        .sum();
    
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_char_add() {
        assert_eq!(Operation::from_char('+'), Some(Operation::Add));
    }

    #[test]
    fn from_char_subtract() {
        assert_eq!(Operation::from_char('-'), Some(Operation::Subtract));
    }

    #[test]
    fn from_char_multiply() {
        assert_eq!(Operation::from_char('*'), Some(Operation::Multiply));
        assert_eq!(Operation::from_char('x'), Some(Operation::Multiply));
    }

    #[test]
    fn from_char_modulo() {
        assert_eq!(Operation::from_char('%'), Some(Operation::Modulo));
    }

    #[test]
    fn from_char_divide() {
        assert_eq!(Operation::from_char('/'), Some(Operation::Divide));
    }

    #[test]
    fn from_char_invalid() {
        assert_eq!(Operation::from_char('a'), None);
        assert_eq!(Operation::from_char('b'), None);
        assert_eq!(Operation::from_char('A'), None);
        assert_eq!(Operation::from_char('B'), None);
        assert_eq!(Operation::from_char('3'), None);
        assert_eq!(Operation::from_char('#'), None);
    }

    #[test]
    fn parse_equation_addition() {
        let equation = "     2.2 + 2.2".to_string();
        assert_eq!(parse_equation(&equation), Ok((2.2, 2.2, Operation::Add)));
    }

    #[test]
    fn parse_equation_subtraction() {
        let equation = "9 -   7.8".to_string();
        assert_eq!(
            parse_equation(&equation),
            Ok((9.0, 7.8, Operation::Subtract))
        );
    }

    #[test]
    fn parse_equation_multiplication_star() {
        let equation = "9.9 * 7          ".to_string();
        assert_eq!(
            parse_equation(&equation),
            Ok((9.9, 7.0, Operation::Multiply))
        );
    }

    #[test]
    fn parse_equation_multiplication_x() {
        let equation = "9.1   x 7.1".to_string();
        assert_eq!(
            parse_equation(&equation),
            Ok((9.1, 7.1, Operation::Multiply))
        );
    }

    #[test]
    fn parse_equation_modulo() {
        let equation = "12 %  6".to_string();
        assert_eq!(
            parse_equation(&equation),
            Ok((12.0, 6.0, Operation::Modulo))
        );
    }

    #[test]
    fn parse_equation_divide() {
        let equation = "12 / 6.01".to_string();
        assert_eq!(
            parse_equation(&equation),
            Ok((12.0, 6.01, Operation::Divide))
        );
    }

    #[test]
    fn parse_equation_invalid_missing_number_left() {
        let equation = " / 12".to_string();
        assert_eq!(parse_equation(&equation), Err(()));
    }

    #[test]
    fn parse_equation_invalid_missing_number_right() {
        let equation = "12 / ".to_string();
        assert_eq!(parse_equation(&equation), Err(()));
    }

    #[test]
    fn parse_equation_invalid_empty() {
        let equation = "".to_string();
        assert_eq!(parse_equation(&equation), Err(()));
    }

    #[test]
    fn solve_equation_addition() {
        let equation = "     2.2 + 2.2".to_string();
        assert_eq!(solve(&equation), Ok(4.4));
    }

    #[test]
    fn solve_equation_subtraction() {
        let equation = "9 -   7.8".to_string();
        assert_eq!(solve(&equation), Ok(1.2));
    }

    #[test]
    fn solve_equation_multiplication_star() {
        let equation = "9.9 * 7          ".to_string();
        assert_eq!(solve(&equation), Ok(69.3));
    }

    #[test]
    fn solve_equation_multiplication_x() {
        let equation = "9.1   x 7.1".to_string();
        assert_eq!(solve(&equation), Ok(64.61));
    }

    #[test]
    fn solve_equation_modulo() {
        let equation = "12 %  6".to_string();
        assert_eq!(solve(&equation), Ok(0.0));
    }

    #[test]
    fn solve_equation_divide() {
        let equation = "12 / 3".to_string();
        assert_eq!(solve(&equation), Ok(4.0));
    }

    #[test]
    fn solve_equation_invalid_missing_number_left() {
        let equation = " / 12".to_string();
        assert_eq!(solve(&equation), Err(()));
    }

    #[test]
    fn solve_equation_invalid_missing_number_right() {
        let equation = "12 / ".to_string();
        assert_eq!(solve(&equation), Err(()));
    }

    #[test]
    fn solve_equation_invalid_empty() {
        let equation = "".to_string();
        assert_eq!(solve(&equation), Err(()));
    }
}
