use spake2::{Ed25519Group, Identity, Password, SPAKE2};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct StartResult {
    state: SPAKE2<Ed25519Group>,
    outbound_msg: Vec<u8>,
}

#[wasm_bindgen]
impl StartResult {
    pub fn get_outbound_msg(&self) -> Vec<u8> {
        self.outbound_msg.clone()
    }
}

#[wasm_bindgen]
pub fn start_a(password: &str, id_a: &str, id_b: &str) -> StartResult {
    let (state, outbound_msg) = SPAKE2::<Ed25519Group>::start_a(
        &Password::new(password.as_bytes()),
        &Identity::new(id_a.as_bytes()),
        &Identity::new(id_b.as_bytes()),
    );
    StartResult {
        state: state,
        outbound_msg: outbound_msg,
    }
}

#[wasm_bindgen]
pub fn start_b(password: &str, id_a: &str, id_b: &str) -> StartResult {
    let (state, outbound_msg) = SPAKE2::<Ed25519Group>::start_b(
        &Password::new(password.as_bytes()),
        &Identity::new(id_a.as_bytes()),
        &Identity::new(id_b.as_bytes()),
    );
    StartResult {
        state: state,
        outbound_msg: outbound_msg,
    }
}

#[wasm_bindgen]
pub fn finish(sr: StartResult, inbound_msg: Vec<u8>) -> Vec<u8> {
    sr.state.finish(&inbound_msg).unwrap()
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
