/*
Given a sorted array of distinct integers and a target value, return the index if the target is found.
If not, return the index where it would be if it were inserted in order.

You must write an algorithm with O(log n) runtime complexity.

Example 1:

Input: nums = [1,3,5,6], target = 5
Output: 2

Example 2:

Input: nums = [1,3,5,6], target = 2
Output: 1

Example 3:

Input: nums = [1,3,5,6], target = 7
Output: 4

*/

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len());
    while left < right {
        let mid = left + ((right - left) / 2);
        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left as _
}


#[allow(unused_imports)]
#[allow(unused_variables)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_insert_1() {
        assert_eq!(search_insert(vec![1,3,5,6],5), 2);
    }

    #[test]
    fn search_insert_2() {
        assert_eq!(search_insert(vec![1,3,5,6],2), 1);
    }

    #[test]
    fn search_insert_3() {
        assert_eq!(search_insert(vec![1,3,5,6],7), 4);
    }
    #[test]
    fn search_insert_4() {
        assert_eq!(search_insert(vec![1,3,5,6],0), 0);
    }

    #[test]
    fn play(){
        let x = vec![1,3, 5, 6];
        let x_s = &x[..1];
        let x_p = &x[..0];
        let l = x_s.len();
    }
}