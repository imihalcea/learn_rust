use std::collections::HashMap;

#[allow(unused_imports)]
#[allow(unused_variables)]

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements = HashMap::new();
    for (i,m) in nums.iter().enumerate() {
        if let Some(j) = complements.get(m){
            return vec![*j as i32, i as i32];
        }
        complements.insert(target - m, i);
    }
    vec![]
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn two_sum_1(){
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn two_sum_2(){
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn two_sum_3(){
        assert_eq!(two_sum(vec![3, 2, 4], 8), vec![]);
    }

}