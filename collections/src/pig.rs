use std::io;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

// 2. Convert strings to pig latin. The first consonant of each
// word is moved to the end of the word and “ay” is added, so
// “first” becomes “irst-fay.” Words that start with a vowel have
// “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

fn word_to_pig_latin(s: &String) -> String {
    if s.len() == 0 {
        return String::new();
    }
    let chars: Vec<char> = s.chars().collect();

    if VOWELS.contains(&chars[0]) {
        format!("{}-hay", s)
    } else {
        let (first, rest) = match chars.split_first() {
            Some(vals) => vals,
            None => return String::new(),
        };

        let converted: String = rest.to_vec().into_iter().collect();
        let converted = format!("{}-{}ay", converted, first);
        converted
    }
}

pub fn driver() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Translate each word into pig latin
        let words: Vec<&str> = input.trim().split(" ").collect();
        let translated_words: Vec<String> = words
            .iter()
            .map(|&word| word_to_pig_latin(&String::from(word)))
            .collect();

        // Print translated words in single input
        let mut translated_input = String::new();
        for (i, word) in translated_words.iter().enumerate() {
            translated_input += word;

            if i != translated_words.len() - 1 {
                translated_input.push(' ');
            }
        }
        println!("{}", translated_input);
    }
}
