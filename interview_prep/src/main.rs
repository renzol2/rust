use std::collections::HashSet;


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
        if *x != 0 { true } else {
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
        let prices = vec![7,1,5,3,6,4];
        let profit = max_profit(prices);
        assert_eq!(profit, 5);
    }

    #[test]
    fn max_profit_returns_no_profit() {
        let prices = vec![7,6,5,4,3,2,1];
        let profit = max_profit(prices);
        assert_eq!(profit, 0);
    }
}
