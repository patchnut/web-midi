//! Wrap web-sys webmidi calls in a more rusty API

pub mod error;

use web_sys::Event;
use js_sys::Array;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{MidiOptions, MidiPortConnectionState, MidiPortDeviceState};

/// This is the main entry point to the web midi library and gives access to midi inputs and outputs
pub struct MidiAccess {
    access: web_sys::MidiAccess,
    callback: Closure<dyn Fn(Event)>
}

impl MidiAccess {
    /// Open a midi MidiAccess instance from a Navigator
    pub async fn get_access(navigator: web_sys::Navigator) -> Self {
        // let window = web_sys::window().expect("no global `window` exists");

        Self::new(
            JsFuture::from(
                navigator
                    .request_midi_access_with_options(MidiOptions::new().sysex(true).software(true))
                    .unwrap(),
            )
            .await
            .unwrap()
            .into(),
        )
    }

    fn new(access: web_sys::MidiAccess) -> Self {
        let callback = Closure::wrap(Box::new(event) as Box<dyn Fn(_)>);

        access.set_onstatechange(Some(callback.as_ref().unchecked_ref()));

        Self { access, callback }
    }

    /// Get all available Midi inputs
    pub fn inputs(&self) -> Vec<MidiInput> {
        js_sys::try_iter(&self.access.inputs())
            .unwrap()
            .unwrap()
            .map(|entry| {
                let array: Array = entry.unwrap().into();
                MidiInput::new(array.get(1).into())
            })
            .collect()
    }

    /// Get all available Midi outputs
    pub fn outputs(&self) -> Vec<MidiOutput> {
        js_sys::try_iter(&self.access.outputs())
            .unwrap()
            .unwrap()
            .map(|entry| {
                let array: Array = entry.unwrap().into();
                MidiOutput::new(array.get(1).into())
            })
            .collect()
    }

    /// Return true if sysex is enabled for this MidiAccess
    pub fn sysex_enabled(&self) -> bool {
        self.access.sysex_enabled()
    }

    // pub fn onstatechange(&self) -> ... {
    //     todo!()
    // }

    // pub fn set_onstatechange(&self) -> ... {
    //     todo!()
    // }
}

fn event(_evt: web_sys::Event) {
    log::info!("state change");
}

pub struct MidiInput {
    input: web_sys::MidiInput,
}

impl MidiInput {
    pub fn new(input: web_sys::MidiInput) -> Self {
        MidiInput { input }
    }

    // pub fn onmidimessage(&self) -> ... {
    //     todo!()
    // }

    // pub fn set_onmidimessage(&self) -> ... {
    //     todo!()
    // }

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

    pub fn state(&self) -> MidiPortDeviceState {
        self.input.state()
    }

    pub fn connection(&self) -> MidiPortConnectionState {
        self.input.connection()
    }

    // pub fn onstatechange(&self) -> ... {
    //     todo!()
    // }

    // pub fn set_onstatechange(&self) -> ... {
    //     todo!()
    // }

    pub async fn open(&self) -> &Self {
        JsFuture::from(self.input.open()).await.unwrap();
        self
    }

    pub async fn close(&self) -> &Self {
        JsFuture::from(self.input.close()).await.unwrap();
        self
    }
}

pub struct MidiOutput {
    output: web_sys::MidiOutput,
}

impl MidiOutput {
    fn new(output: web_sys::MidiOutput) -> Self {
        Self { output }
    }

    /// Clear any pending data that has not yet been sent from the MidiOutput's queue
    pub fn clear(&self) {
        self.output.clear()
    }

    /// Enqueue a message to be sent to the corresponding MIDI port.
    /// The data contains one or more valid, complete MIDI messages. Running status is not allowed in the data, as underlying systems may not support it.
    pub fn send(&self, data: &JsValue) -> Result<(), JsValue> {
        // TODO: fix input and output types to be something sensible
        self.output.send(data)
    }

    /// Enqueue a message to be sent to the corresponding MIDI port.
    /// The data contains one or more valid, complete MIDI messages. Running status is not allowed in the data, as underlying systems may not support it.
    pub fn send_with_timestamp(&self, data: &JsValue, timestamp: f64) -> Result<(), JsValue> {
        // TODO: fix input and output types to be something sensible
        self.output.send_with_timestamp(data, timestamp)
    }

    /// A unique ID of the port. This can be used by developers to remember ports the user has chosen for their application.
    pub fn id(&self) -> String {
        self.output.id()
    }

    /// The manufacturer of the port
    pub fn manufacturer(&self) -> Option<String> {
        self.output.manufacturer()
    }

    /// The system name of the port
    pub fn name(&self) -> Option<String> {
        self.output.name()
    }

    /// The version of the port.
    pub fn version(&self) -> Option<String> {
        self.output.version()
    }

    /// The state of the device
    pub fn state(&self) -> MidiPortDeviceState {
        self.output.state()
    }

    /// The state of the connection to the device.
    pub fn connection(&self) -> MidiPortConnectionState {
        self.output.connection()
    }

    // pub fn onstatechange(&self) -> ... {
    //     todo!()
    // }

    // pub fn set_onstatechange(&self) -> ... {
    //     todo!()
    // }

    /// Make the MIDI device corresponding to the MIDIPort explicitly available. Note that this call is NOT required in order to use the MIDIPort - calling send() on a MIDIOutput or attaching a MIDIMessageEvent handler on a MIDIInputPort will cause an implicit open().
    pub async fn open(&self) -> &Self {
        JsFuture::from(self.output.open()).await.unwrap();
        self
    }

    /// Make the MIDI device corresponding to the MIDIPort explicitly unavailable (subsequently changing the state from "open" to "connected"). Note that successful invocation of this method will result in MIDI messages no longer being delivered to MIDIMessageEvent handlers on a MIDIInputPort (although setting a new handler will cause an implicit open()).
    pub async fn close(&self) -> &Self {
        JsFuture::from(self.output.close()).await.unwrap();
        self
    }
}
