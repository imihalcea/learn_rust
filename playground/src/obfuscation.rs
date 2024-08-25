pub struct Obfuscation {
    pub value: String,
    pub start: usize,
    pub end: usize,
}

pub struct Unobfuscation {
    pub obfuscation_value: String,
    pub readable_value: String,
}

pub fn unobfuscate(obfuscated: &str, substitutions: Vec<Unobfuscation>) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    for replacement in substitutions {
        let pos = obfuscated.find(&replacement.obfuscation_value);
        if pos.is_none() {
            continue;
        }
        result.push_str(&obfuscated[last_end..obfuscated.find(&replacement.obfuscation_value).unwrap()]);
        result.push_str(&replacement.readable_value);
        last_end = obfuscated.find(&replacement.obfuscation_value).unwrap() + replacement.obfuscation_value.len();
    }
    result.push_str(&obfuscated[last_end..]);
    result
}

pub fn unobfuscate_in_place(obfuscated: &mut String, substitutions: Vec<Unobfuscation>) {
    for replacement in substitutions {
        let mut start = 0;
        while let Some(pos) = obfuscated[start..].find(&replacement.obfuscation_value) {
            let start_pos = start + pos;
            let end_pos = start_pos + replacement.obfuscation_value.len();
            obfuscated.replace_range(start_pos..end_pos, &replacement.readable_value);
            start = start_pos + replacement.readable_value.len();
        }
    }
}
pub fn obfuscate(original:String, substitutions: Vec<Obfuscation>) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    for replacement in substitutions {
        result.push_str(&original[last_end..replacement.start]);
        result.push_str(&replacement.value);
        last_end = replacement.end;
    }
    result.push_str(&original[last_end..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obfuscation_nominal() {
        let original = "John Doe, Jane Doe are happy.".to_string();
        let substitutions = vec![
            Obfuscation { value: "X".to_string(), start: 0, end: 8},
            Obfuscation { value: "Y".to_string(), start: 10, end: 18},
        ];
        let result = obfuscate(original, substitutions);
        assert_eq!(result, "X, Y are happy.");
    }

    #[test]
    fn unobfuscation_in_place_nominal() {
        let mut obfuscated = "X, Y are happy.".to_string();
        let substitutions = vec![
            Unobfuscation { obfuscation_value: "X".to_string(), readable_value: "John Doe".to_string()},
            Unobfuscation { obfuscation_value: "Y".to_string(), readable_value: "Jane Doe".to_string()},
        ];
        unobfuscate_in_place(&mut obfuscated, substitutions);
        assert_eq!(obfuscated, "John Doe, Jane Doe are happy.");
    }

    #[test]
    fn unobfuscation_nominal() {
        let obfuscated = "X, Y are happy.".to_string();
        let substitutions = vec![
            Unobfuscation { obfuscation_value: "X".to_string(), readable_value: "John Doe".to_string()},
            Unobfuscation { obfuscation_value: "Y".to_string(), readable_value: "Jane Doe".to_string()},
        ];
        let result = unobfuscate(&obfuscated, substitutions);
        assert_eq!(result, "John Doe, Jane Doe are happy.");
    }
}
