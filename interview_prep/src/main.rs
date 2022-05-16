use std::collections::HashMap;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut frequencies: HashMap<i32, u32> = HashMap::new();
    for num in nums {
        match frequencies.get(&num) {
            Some(freq) => {
                let freq = freq + 1;
                frequencies.insert(num, freq + 1);
            }
            None => {
                frequencies.insert(num, 1);
            }
        };
    }

    for (_, frequency) in frequencies {
        if frequency > 1 {
            return true;
        }
    }

    return false;
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_duplicate_finds_duplicate_correctly() {
        let v = vec![1, 2, 3, 4, 3, 5];
        assert!(contains_duplicate(v));
    }

    #[test]
    fn contains_duplicate_finds_no_duplicate_correctly() {
        let v = vec![1, 2, 3, 4, 5];
        assert!(!contains_duplicate(v));
    }
}