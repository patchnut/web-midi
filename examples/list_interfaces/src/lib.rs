use async_std::task;
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_midi::MidiAccess;

#[wasm_bindgen]
pub fn main_js() -> Result<(), JsValue> {

    task::block_on(async {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let access = MidiAccess::get_access().await;

        for input in access.inputs() {

        }

        for output in access.outputs() {
            
        }

        // val.set_inner_html(&format!("{:?}", access.inputs()));
        // body.append_child(&val).unwrap();
    });

    Ok(())
}
