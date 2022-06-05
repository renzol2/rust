use std::collections::{HashMap, HashSet};

/// Given an integer array `nums`, return `true` if any value appears at least
/// twice in the array, and return `false` if every element is distinct.
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut num_set = HashSet::new();
    for num in nums {
        if !num_set.insert(num) {
            return true;
        }
    }

    return false;
}

/// Given two integer arrays `nums1` and `nums2`, return an array of their intersection.
/// Each element in the result must be unique and you may return the result in any order.
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut set1 = HashSet::new();
    for n in nums1 {
        set1.insert(n);
    }

    let mut set2 = HashSet::new();

    for n in nums2 {
        if set1.contains(&n) {
            set2.insert(n);
        }
    }

    let v: Vec<i32> = set2.into_iter().collect();
    v
}

///  Given an integer array nums, move all 0's to the end of it while maintaining the relative
///  order of the non-zero elements.
///
///  Note that you must do this in-place without making a copy of the array.
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut z = Vec::new();
    nums.retain(|x| {
        if *x != 0 {
            true
        } else {
            z.push(0);
            false
        }
    });
    nums.append(&mut z);
}

/// You are given an array prices where ` is the price of a given
/// stock on the `th day.
///
/// You want to maximize your profit by choosing a single day to buy one
/// stock and choosing a different day in the future to sell that stock.
///
/// Return the maximum profit you can achieve from this transaction.
/// If you cannot achieve any profit, return 0.
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut min_price = 10_000;
    for price in prices {
        if price < min_price {
            min_price = price;
        } else if profit < price - min_price {
            profit = price - min_price;
        }
    }
    profit
}

/// Given two strings `s` and `t`, return `true` if `t` is an anagram
/// of `s`, and `false` otherwise.
///
/// An **Anagram** is a word or phrase formed by rearranging the letters
/// of a different word or phrase, typically using all the original
/// letters exactly once.
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut freqs: HashMap<char, u32> = HashMap::new();
    for c in s.chars() {
        let counter = freqs.entry(c).or_insert(0);
        *counter += 1;
    }

    for c in t.chars() {
        if freqs.contains_key(&c) {
            let counter = freqs.entry(c).or_insert(0);
            if *counter == 0 {
                return false;
            } else {
                *counter -= 1;
            }
        } else {
            return false;
        }
    }

    true
}

struct MyStack {
    stack: Vec<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack { stack: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    fn peek(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap()
    }

    fn size(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.len() == 0
    }
}

struct MyQueue {
    main_stack: MyStack,
    transfer_stack: MyStack,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            main_stack: MyStack::new(),
            transfer_stack: MyStack::new(),
        }
    }

    fn push(&mut self, x: i32) {
        // Pop everything from main stack and push into transfer stack
        while !self.main_stack.is_empty() {
            self.transfer_stack.push(self.main_stack.pop());
        }

        // Push x into main stack
        self.main_stack.push(x);

        // Pop everything back from transfer to main stack
        while !self.transfer_stack.is_empty() {
            self.main_stack.push(self.transfer_stack.pop());
        }
    }

    fn pop(&mut self) -> i32 {
        self.main_stack.pop()
    }

    fn peek(&self) -> i32 {
        self.main_stack.peek()
    }

    fn empty(&self) -> bool {
        self.main_stack.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

fn main() {
    contains_duplicate(vec![1, 2, 3, 4, 5]);
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

    #[test]
    fn intersection_no_intersection() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6];
        assert!(intersection(nums1, nums2).len() == 0);
    }

    #[test]
    fn intersection_finds_intersection_correctly() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![3, 5, 6];
        let expected = vec![3];
        assert!(intersection(nums1, nums2) == expected);
    }

    #[test]
    fn move_zeroes_moves_all_zeroes_correctly() {
        let mut nums = vec![0, 1, 0, 0, 2, 3];
        let expected = vec![1, 2, 3, 0, 0, 0];
        move_zeroes(&mut nums);
        assert!(nums == expected);
    }

    #[test]
    fn move_zeroes_does_nothing_with_no_zeroes() {
        let mut nums = vec![4, 3, 2, 1, 2, 3];
        let expected = vec![4, 3, 2, 1, 2, 3];
        move_zeroes(&mut nums);
        assert!(nums == expected);
    }

    #[test]
    fn max_profit_returns_max_profit() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let profit = max_profit(prices);
        assert_eq!(profit, 5);
    }

    #[test]
    fn max_profit_returns_no_profit() {
        let prices = vec![7, 6, 5, 4, 3, 2, 1];
        let profit = max_profit(prices);
        assert_eq!(profit, 0);
    }

    #[test]
    fn is_anagram_finds_anagram_correctly() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        assert_eq!(is_anagram(s, t), true);
    }

    #[test]
    fn is_anagram_finds_non_anagram_correctly() {
        let s = "rat".to_string();
        let t = "car".to_string();
        assert_eq!(is_anagram(s, t), false);
    }

    #[test]
    fn is_anagram_finds_double_letter_non_anagram_correctly() {
        let s = "rat".to_string();
        let t = "rar".to_string();
        assert_eq!(is_anagram(s, t), false);
    }
}
