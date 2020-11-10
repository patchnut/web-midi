use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, MidiOptions};

pub struct MidiAccess {
    access: web_sys::MidiAccess,
}

impl MidiAccess {
    pub async fn get_access() -> Self {
        let window = web_sys::window().expect("no global `window` exists");

        let access: web_sys::MidiAccess = JsFuture::from(
            window
                .navigator()
                .request_midi_access_with_options(MidiOptions::new().sysex(true).software(true))
                .unwrap(),
        )
        .await
        .unwrap()
        .into();

        Self { access }
    }

    pub fn inputs(&self) -> Vec<MidiInput> {
        let map: js_sys::Map = self.access.inputs().dyn_into().unwrap();

        for entry in js_sys::try_iter(&map.values()).unwrap().unwrap() {
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
