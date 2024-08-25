use std::collections::HashMap;

#[allow(unused_imports)]
#[allow(unused_variables)]


pub fn is_valid(s: String) -> bool {
    let map: HashMap<char, char> = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
    let mut stack = Vec::new();

    for c in s.chars() {
        if map.contains_key(&c) {
            stack.push(c);
        } else if let Some(&last) = stack.last() {
            if map.get(&last) == Some(&c) {
                stack.pop();
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    stack.is_empty()
}

pub fn is_valid2(s:String) -> bool{
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '}'|')'|']' if Some(c) != stack.pop() => return false,
            _ => ()
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn is_valid_1(){
        assert_eq!(is_valid("()".to_string()), true);
    }

    #[test]
    fn is_valid_2(){
        assert_eq!(is_valid2("()[]{}".to_string()), true);
    }

    #[test]
    fn is_valid_3(){
        assert_eq!(is_valid2("(]".to_string()), false);
    }

    #[test]
    fn is_valid_4(){
        assert_eq!(is_valid2("([])".to_string()), true);
    }

    #[test]
    fn is_valid_5(){
        //"(){}}{"
        assert_eq!(is_valid2("(){}}{".to_string()), false);
    }

}