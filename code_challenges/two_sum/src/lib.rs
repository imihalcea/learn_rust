/**

//https://leetcode.com/problems/two-sum/

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

2 <= nums.length <= 10^4
-10^9 <= nums[i] <= 10^9
-10^9 <= target <= 10^9
 */

use std::collections::HashMap;

pub fn optimized(nums: Vec<i32>, target: i32) -> Vec<i32>
{
    let mut complements = HashMap::new();
    for (i, m) in nums.iter().enumerate() {
        if let Some(j) = complements.get(m) {
            return vec![*j as i32, i as i32];
        }
        complements.insert(target - m, i);
    }
    vec![]
}


pub fn brute_force(nums: Vec<i32>, target: i32) -> Vec<i32>
{
    for (i, m) in nums.iter().enumerate() {
        for (j, n) in nums.iter().enumerate() {
            if i == j {
                continue;
            }
            if m + n == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::str::FromStr;
    use test::Bencher;
    use super::*;

    #[test]
    fn test_brute_force() {
        let result = brute_force(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);

        let result = brute_force(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);

        let result = brute_force(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn example2() {
        let result = optimized(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);

        let result = optimized(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);

        let result = optimized(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}

