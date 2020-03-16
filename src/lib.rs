use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

#[wasm_bindgen]
pub fn fizzbuzz(n: u32) -> String {
    match (n%3, n%5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => n.to_string(),
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    log("Hello, world!");

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let app = document.get_element_by_id("app").unwrap();
    app.set_inner_html("Hello from Rust!");

    let p = document.create_element("p").unwrap();
    p.set_inner_html("色は匂へど散りぬるを");
    app.append_child(&p).unwrap();
}
