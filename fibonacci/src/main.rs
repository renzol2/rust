use std::io;

fn main() {
    loop {
        println!("\nGenerate the nth fibonacci number:\n");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Error reading input");
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("By DP, the {}th fibonacci number is {}", n, fib_dp(n));
        println!("By recursion, the {}th fibonacci number is {}", n, fib(n));
    }
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn fib_dp(n: u32) -> u32 {
    if n == 0 {
        return 0;
    };
    if n == 1 {
        return 1;
    };

    // not sure what the highest fibonacci number is that's
    // representable by u32 number, but we'll find out i guess
    let mut storage = [0; 48];
    storage[0] = 0;
    storage[1] = 1;
    let n: usize = n as usize;
    for i in 2..=n as usize {
        storage[i] = storage[i - 1] + storage[i - 2];
    }

    storage[n]
}
