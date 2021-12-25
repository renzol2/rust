fn main() {
    let s = String::from("hello");

    takes_ownership(s);
    // print not possible, since s's value moved to the function
    // println!("{}", s);

    let x = 5;

    makes_copy(x);

    // possible! since i32 has Copy trait
    println!("{}", x)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
