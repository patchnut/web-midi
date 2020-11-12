use async_std::task;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_midi::MidiAccess;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn main_js() -> Result<(), JsValue> {

    task::block_on(async {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let access = MidiAccess::get_access().await;

        for input in access.inputs() {
            console!("input {}", input.name().unwrap())
        }

        for output in access.outputs() {
            console!("output {}", output.name().unwrap())
        }
    });

    Ok(())
}


