use wasm_bindgen::prelude::*;

use zeblang;
mod interpret;
use interpret::interpret;

#[wasm_bindgen]
pub fn do_compile(src: &str) -> String {
    return compile(src);
}

pub fn compile(src: &str) -> String {
    match zeblang::make_parsetree(src.to_string()) {
        Ok(out) => match interpret(out) {
            Ok(value) => format!("{}", value),
            Err(error) => error,
        },
        Err(_) => "error".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::compile;

    #[test]
    fn test_while() {
        let src = r#"
i = 10
out = 0
while i
  out = out + 2
  j = 2
  while j 
    out = out + 1
    j = j -1 
  elihw
  i = i - 1
elihw
exit out
"#;
        assert_eq!(compile(src), "40".to_string());
    }
}
