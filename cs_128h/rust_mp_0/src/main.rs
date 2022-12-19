extern crate dummy_project;

use text_io::read;

fn main() {
    let num = dummy_project::solve_all("equations.txt").unwrap();
    println!("Equation file solved: {:?}", num);
    println!("🧮 Please enter an equation. Enter 'done' to exit the program.");
    let mut line: String = read!("{}\n");

    while line != "done" {
        println!("Solution: {:?}", dummy_project::solve(&line).unwrap());

        println!("🧮 Please enter an equation. Enter 'done' to exit the program.");
        line = read!("{}\n");
    }
}