use js_sys::Array;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::MidiOptions;

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

impl MidiInput {
    pub fn id(&self) -> String {
        self.input.id()
    }

    pub fn manufacturer(&self) -> Option<String> {
        self.input.manufacturer()
    }

    pub fn name(&self) -> Option<String> {
        self.input.name()
    }

    pub fn version(&self) -> Option<String> {
        self.input.version()
    }

    // pub fn state(&self) -> MidiPortDeviceState {
    //     self.input.state()
    // }

    // pub fn connection(&self) -> MidiPortConnectionState {
    //     self.input.connection()
    // }

    // pub fn onstatechange(&self) -> ... {
    //     todo!()
    // }

    // pub fn set_onstatechange(&self) -> ... {
    //     todo!()
    // }

    pub async fn open(&self) -> JsValue {
        JsFuture::from(self.input.open()).await.unwrap()
    }

    pub async fn close(&self) -> JsValue {
        JsFuture::from(self.input.close()).await.unwrap()
    }
}

pub struct MidiOutput {
    output: web_sys::MidiOutput,
}

impl MidiOutput {
    pub fn id(&self) -> String {
        self.output.id()
    }

    pub fn manufacturer(&self) -> Option<String> {
        self.output.manufacturer()
    }

    pub fn name(&self) -> Option<String> {
        self.output.name()
    }

    pub fn version(&self) -> Option<String> {
        self.output.version()
    }

    // pub fn state(&self) -> MidiPortDeviceState {
    //     self.output.state()
    // }

    // pub fn connection(&self) -> MidiPortConnectionState {
    //     self.output.connection()
    // }

    // pub fn onstatechange(&self) -> ... {
    //     todo!()
    // }

    // pub fn set_onstatechange(&self) -> ... {
    //     todo!()
    // }

    pub async fn open(&self) -> JsValue {
        JsFuture::from(self.output.open()).await.unwrap()
    }

    pub async fn close(&self) -> JsValue {
        JsFuture::from(self.output.close()).await.unwrap()
    }
}
