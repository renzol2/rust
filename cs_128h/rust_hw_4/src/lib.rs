/// You must iterate through the vector.
/// If the word has an even number of characters, reverse the word by using the reverse function.
/// If the word has an odd number of characters, count the number of vowels in the word.
/// You should return a tuple containg a Vector of the reversed words
/// and the total number of vowels from odd lengthed words.
pub fn vector_iter(words: &mut Vec<String>) -> (Vec<String>, i32) {
    let mut reversed_words = Vec::new();
    let mut vowel_count = 0;
    for word in words {
        if word.len() % 2 == 0 {
            reverse_word(word);
            reversed_words.push(word.clone());
        } else {
           vowel_count += count_vowels(word);
        }
    }
    (reversed_words, vowel_count)
}

/// This should directly modify the contents of the passed String.
pub fn reverse_word(word: &mut String) {
    unsafe {
        let bytes: &mut [u8] = word.as_bytes_mut();
        let len = bytes.len();
        for i in 0..len / 2 {
            let temp = bytes[i];
            bytes[i] = bytes[len - 1 - i];
            bytes[len - 1 - i] = temp;
        }
    }
}

/// TODO: Implement this function that takes a String and counts the number of vowels in it.
/// In this instance, we mean the vowels a, e, i, o, and u. Do not worry about y.
/// You will need to handle words with upper and lower case vowels.
/// You should not modify the contents of the passed String.
/// You should return the vowel count.
pub fn count_vowels(word: &str) -> i32 {
    let mut vowel_count = 0;
    for c in word.to_lowercase().chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => {}
        }
    }
    vowel_count
}

// You can test your code with the test cases we've provided by running `cargo test`
// We also encourage you to add more assert statements to test out your code further!
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_iter() {
        let words: &mut Vec<String> = &mut vec![
            "fish".to_string(),
            "cat".to_string(),
            "ardvaark".to_string(),
            "dog".to_string(),
            "bird".to_string(),
            "ant".to_string(),
        ];
        let (reversed_words, count) = vector_iter(words);
        assert_eq!(
            reversed_words,
            vec![
                "hsif".to_string(),
                "kraavdra".to_string(),
                "drib".to_string()
            ]
        );
        assert_eq!(count, 3);
    }

    #[test]
    fn test_reverse_word() {
        let mut s = "music".to_string();
        reverse_word(&mut s);
        assert_eq!(s, "cisum");
    }

    #[test]
    fn test_count_vowels() {
        let mut s = "music";
        let vowel_count = count_vowels(&mut s);
        assert_eq!(vowel_count, 2);
    }

    #[test]
    fn test_count_vowels_all_vowels() {
        let mut s = "aeiou";
        let vowel_count = count_vowels(&mut s);
        assert_eq!(vowel_count, 5);
    }

    #[test]
    fn test_count_vowels_no_vowels() {
        let mut s = "cdpsrwz";
        let vowel_count = count_vowels(&mut s);
        assert_eq!(vowel_count, 0);
    }
}
