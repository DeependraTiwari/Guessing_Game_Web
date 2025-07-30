use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub fn check_guess(user_guess: u32, secret: u32) -> String {
    if user_guess < secret {
        "Too small!".into()
    } else if user_guess > secret {
        "Too big!".into()
    } else {
        "You win!".into()
    }
}

#[wasm_bindgen]
pub fn generate_secret(max: u32) -> u32 {
    rand::thread_rng().gen_range(1..=max)
}
