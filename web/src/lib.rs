use wasm_bindgen::prelude::*;

use pi_collision::Collisions;


#[wasm_bindgen]
pub fn gen_svg(mass_big: f64) -> String {
    let mut out = Vec::new();
    Collisions::calculate(mass_big).write_svg(&mut out).unwrap();

    String::from_utf8(out).unwrap()
}
