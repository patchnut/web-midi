use async_std::task;
use log::{info, Level};
use wasm_bindgen::prelude::*;
use web_midi::MidiAccess;

#[wasm_bindgen]
pub fn main_js() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Debug).expect("Unable to initialize logging");

    task::block_on(async {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let _body = document.body().expect("document should have a body");

        let access = MidiAccess::get_access(window.navigator()).await;

        for input in access.inputs() {
            input.open().await;
            info!(
                "input {} {} {}",
                input.name().unwrap(),
                input.manufacturer().unwrap(),
                input.version().unwrap()
            );
        }

        for output in access.outputs() {
            info!(
                "output {} {} {}",
                output.name().unwrap(),
                output.manufacturer().unwrap(),
                output.version().unwrap()
            );
        }
    });

    Ok(())
}
