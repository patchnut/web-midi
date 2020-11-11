use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, MidiOptions};
use js_sys::Array;

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
        console::log_1(&self.access.inputs());
        for entry in js_sys::try_iter(&self.access.inputs()).unwrap().unwrap() {
            // console::log_1(&entry.unwrap());
            let array: Array = entry.unwrap().into();
            console::log_1(&array.get(1));
        }

        Vec::new()
    }

    pub fn outputs(&self) -> Vec<MidiOutput> {
        // let map: js_sys::Map = self.access.outputs().dyn_into().unwrap();

        // for entry in js_sys::try_iter(&map.keys()).unwrap().unwrap() {
        //     console::log_1(&entry.unwrap());
        // }

        Vec::new()
    }
}

pub struct MidiInput {
    input: web_sys::MidiInput,
}

pub struct MidiOutput {
    output: web_sys::MidiOutput,
}
