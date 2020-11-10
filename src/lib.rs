use web_sys::{MidiOptions, console};
use wasm_bindgen_futures::JsFuture;

pub struct MidiAccess {
    access: web_sys::MidiAccess
}

impl MidiAccess {
    pub async fn get_access() -> Self {
        let window = web_sys::window().expect("no global `window` exists");

        let access: web_sys::MidiAccess = JsFuture::from(
            window.navigator()
                .request_midi_access_with_options(MidiOptions::new().sysex(true).software(true))
                .unwrap(),
        )
        .await
        .unwrap()
        .into();

        Self { access }
    }

    pub fn inputs(&self) -> Vec<MidiInput> {
        for entry in js_sys::try_iter(&self.access.inputs()).unwrap().unwrap() {
            console::log_1(&entry.unwrap());
        }

        Vec::new()
    }

    pub fn outputs(&self) -> Vec<MidiOutput> {
        for entry in js_sys::try_iter(&self.access.inputs()).unwrap().unwrap() {
            console::log_1(&entry.unwrap());
        }

        Vec::new()
    }
}

pub struct MidiInput {
    input: web_sys::MidiInput,
}

pub struct MidiOutput {
    output: web_sys::MidiOutput,
}
