use ed25519_zebra::{Signature, SigningKey, VerificationKey};
use rand::thread_rng;
use std::convert::TryFrom;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(js_name = keyPair)]
pub fn key_pair() -> JsValue {
    let sk = SigningKey::new(thread_rng());
    let vk = VerificationKey::from(&sk);
    serde_wasm_bindgen::to_value(&[<[u8; 32]>::from(sk).to_vec(), <[u8; 32]>::from(vk).to_vec()])
        .unwrap()
}

#[wasm_bindgen]
pub fn sign(message: &[u8], sk_bytes: &[u8]) -> JsValue {
    let sk = SigningKey::from(<[u8; 32]>::try_from(sk_bytes).unwrap());
    let sig = sk.sign(message);
    serde_wasm_bindgen::to_value(&<[u8; 64]>::from(sig).to_vec()).unwrap()
}

#[wasm_bindgen]
pub fn verify(message: &[u8], sig_bytes: &[u8], vk_bytes: &[u8]) -> JsValue {
    let mut sig_bytes_fixed: [u8; 64] = [0; 64];
    sig_bytes_fixed.copy_from_slice(&sig_bytes);
    VerificationKey::try_from(vk_bytes)
        .and_then(|vk| vk.verify(&Signature::from(sig_bytes_fixed), message))
        .is_ok()
        .into()
}
