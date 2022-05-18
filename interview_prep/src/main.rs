use std::collections::HashSet;

/**
 Given an integer array `nums`, return `true` if any value appears at least
 twice in the array, and return `false` if every element is distinct.
*/
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut num_set = HashSet::new();
    for num in nums {
        if !num_set.insert(num) {
            return true;
        }
    }

    return false;
}

/**
 Given two integer arrays `nums1` and `nums2`, return an array of their intersection.
 Each element in the result must be unique and you may return the result in any order.
*/
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
}
