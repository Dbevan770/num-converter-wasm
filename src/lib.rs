use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn num_to_string(num: u8) -> String {
    if num >= 10 {
        return num.to_string();
    }

    match num {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => "NaN".to_string(),
    }
}
