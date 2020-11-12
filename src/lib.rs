use js_sys::Array;
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
        let mut result: Vec<MidiInput> = Vec::new();

        for entry in js_sys::try_iter(&self.access.inputs()).unwrap().unwrap() {
            let array: Array = entry.unwrap().into();
            result.push(MidiInput {
                input: array.get(1).into(),
            });
        }

        result
    }

    pub fn outputs(&self) -> Vec<MidiOutput> {
        let mut result: Vec<MidiOutput> = Vec::new();

        for entry in js_sys::try_iter(&self.access.inputs()).unwrap().unwrap() {
            let array: Array = entry.unwrap().into();
            result.push(MidiOutput {
                output: array.get(1).into(),
            });
        }

        result
    }
}

pub struct MidiInput {
    input: web_sys::MidiInput,
}

pub struct MidiOutput {
    output: web_sys::MidiOutput,
}
