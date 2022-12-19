use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo
}

impl Operation {
    // [COMPLETE THIS FUNCTION]
    pub fn from_char(symbol: char) -> Option<Operation> {
        todo!();
    }
}

// [HELPER FUNCTION - DO NOT EDIT]
pub fn get_equation_tuple(line: &String) -> (Option<&str>, Option<&str>) {
    let v: Vec<&str> = line.split(&['-', '+', 'x', '*', '/', '%'][..]).collect();
    
    (v.get(0).cloned(), v.get(1).cloned())
}

// [COMPLETE THIS HELPER FUNCTION]
pub fn parse_equation(line: &String) -> Result<(f32, f32, Operation), ()> {
    todo!();
}

// [COMPLETE THIS FUNCTION]
pub fn solve(equation: &String) -> Result<f32, ()> {
    todo!();
}

// [COMPLETE THIS FUNCTION]
pub fn solve_all(file_path: &str) -> Result<f32, ()> {
    todo!();
}