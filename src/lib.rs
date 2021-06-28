use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn log_in_rust() {
    console::log_1(&"Hello World".into());
}

#[wasm_bindgen]
pub fn add_two_ints(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn resize_window() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    window.set_inner_height(&"500".into()).unwrap();
    window.set_inner_width(&"500".into()).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_two_ints(2, 2), 4);
    }
}
