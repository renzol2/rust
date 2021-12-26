mod statistics;

// Collections Exercises
// https://doc.rust-lang.org/book/ch08-03-hash-maps.html

// 2. Convert strings to pig latin. The first consonant of each
// word is moved to the end of the word and “ay” is added, so
// “first” becomes “irst-fay.” Words that start with a vowel have
// “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

// 3. Using a hash map and vectors, create a text interface to allow
// a user to add employee names to a department in a company. For
// example, “Add Sally to Engineering” or “Add Amir to Sales.” Then
// let the user retrieve a list of all people in a department or all
// people in the company by department, sorted alphabetically.

fn main() {
    statistics::driver();
}
