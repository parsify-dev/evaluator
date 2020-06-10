use regex::Regex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn remove_comments(expression: &str) -> String {
    let re = Regex::new(r"((/{2}|#).*)").unwrap();
    let result = re.replace_all(expression, "");

    result.trim().to_string()
}

#[wasm_bindgen]
pub fn remove_labels(expression: &str) -> String {
    let re = Regex::new(r".*:").unwrap();
    let result = re.replace_all(expression, "");

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_conversion() {
        assert_eq!(remove_comments("2+2 // quick maths"), "2+2");
    }

    #[test]
    fn from_eq_base() {
        assert_eq!(remove_labels("Quick maths: 2+2"), "2+2");
    }
}
