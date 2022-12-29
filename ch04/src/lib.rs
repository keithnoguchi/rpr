use wasm_bindgen::prelude::wasm_bindgen;

// import window.alert.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// export hello function.
#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("Hello {name}"));
}
