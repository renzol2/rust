use std::io;

// 2. Convert strings to pig latin. The first consonant of each
// word is moved to the end of the word and “ay” is added, so
// “first” becomes “irst-fay.” Words that start with a vowel have
// “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

fn to_pig_latin(s: &String) -> String {
    let c: Vec<char> = s.chars().collect();
    String::from("")
    // if first character is consonant
    // else (if first char is vowel)
}

pub fn driver() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // TODO: split input into words (by space?)
        // TODO: convert each word into pig latin
        println!("{}", input);
    }
}
