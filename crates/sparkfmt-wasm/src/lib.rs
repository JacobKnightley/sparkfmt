use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn format_sql(input: &str) -> String {
    // Set up panic hook for better error messages
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    // Format the SQL, returning original input on error
    match sparkfmt_core::format_sql(input) {
        Ok(formatted) => formatted,
        Err(err) => {
            // Log error to console for debugging (WASM only)
            #[cfg(target_arch = "wasm32")]
            {
                error(&format!("sparkfmt parse error: {}", err.message));
            }
            
            // Suppress unused variable warning for non-wasm builds
            #[cfg(not(target_arch = "wasm32"))]
            let _ = err;
            
            input.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_sql() {
        let input = "select a from t";
        let result = format_sql(input);
        assert!(result.contains("SELECT"));
    }
}
