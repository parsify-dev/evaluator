use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate(expression: &str) -> f64 {
    meval::eval_str(expression).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn general() {
        assert_eq!(evaluate("14+5"), 19.0);
    }
}
