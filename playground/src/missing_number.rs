#[allow(unused_imports)]
#[allow(unused_variables)]


/*
Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.



Example 1:

Input: nums = [3,0,1]
Output: 2
Explanation: n = 3 since there are 3 numbers, so all numbers are in the range [0,3]. 2 is the missing number in the range since it does not appear in nums.

Example 2:

Input: nums = [0,1]
Output: 2
Explanation: n = 2 since there are 2 numbers, so all numbers are in the range [0,2]. 2 is the missing number in the range since it does not appear in nums.

Example 3:

Input: nums = [9,6,4,2,3,5,7,0,1]
Output: 8
Explanation: n = 9 since there are 9 numbers, so all numbers are in the range [0,9]. 8 is the missing number in the range since it does not appear in nums.

*/

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = (nums.len() + 1) as i32;
    (n * (n-1))/2 - nums.iter().sum::<i32>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_number_1() {
        assert_eq!(missing_number(vec![3,0,1]), 2);
    }

    #[test]
    fn missing_number_2() {
        assert_eq!(missing_number(vec![0,1]), 2);
    }

    #[test]
    fn missing_number_3() {
        assert_eq!(missing_number(vec![9,6,4,2,3,5,7,0,1]), 8);
    }
}
