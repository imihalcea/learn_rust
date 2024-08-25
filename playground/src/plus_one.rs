/*

You are given a large integer represented as an integer array digits,
where each digits[i] is the ith digit of the integer.
The digits are ordered from most significant to least significant in left-to-right order.
The large integer does not contain any leading 0's.

Increment the large integer by one and return the resulting array of digits.

Example 1:

Input: digits = [1,2,3]
Output: [1,2,4]
Explanation: The array represents the integer 123.
Incrementing by one gives 123 + 1 = 124.
Thus, the result should be [1,2,4].

Example 2:

Input: digits = [4,3,2,1]
Output: [4,3,2,2]
Explanation: The array represents the integer 4321.
Incrementing by one gives 4321 + 1 = 4322.
Thus, the result should be [4,3,2,2].

Example 3:

Input: digits = [9]
Output: [1,0]
Explanation: The array represents the integer 9.
Incrementing by one gives 9 + 1 = 10.
Thus, the result should be [1,0].



Constraints:

    1 <= digits.length <= 100
    0 <= digits[i] <= 9

*/



pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    for i in (0..digits.len()).rev() {
        digits[i] = (digits[i] + 1) % 10;
        if digits[i] != 0 {
            return digits;
        }
        if i == 0 {
            digits.insert(0, 1);
            return digits;
        }
    }
    digits
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plus_one_1() {
        assert_eq!(plus_one(vec![1,2,3]), vec![1,2,4]);
    }

    #[test]
    fn plus_one_2() {
        assert_eq!(plus_one(vec![9,8,7,6,5,4,3,2,1,0]), vec![9,8,7,6,5,4,3,2,1,1]);
    }

    #[test]
    fn plus_one_3() {
        assert_eq!(plus_one(vec![9]), vec![1,0]);
    }

    #[test]
    fn plus_one_4() {
        assert_eq!(plus_one(vec![9,9,9]), vec![1,0,0,0]);
    }

    #[test]
    fn plus_one_5() {
        assert_eq!(plus_one(vec![9,9,8]), vec![9,9,9]);
    }

    #[test]
    fn plus_one_6() {
        assert_eq!(plus_one(vec![8,9,9]), vec![9,0,0]);
    }
}