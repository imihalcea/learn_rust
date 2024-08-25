#[allow(unused_imports)]
#[allow(unused_variables)]


pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for num in nums {
        result ^= num;
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number_1() {
        assert_eq!(single_number(vec![2,2,1]), 1);
    }
}