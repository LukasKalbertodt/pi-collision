cargo +nightly build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/pi_collision_web.wasm \
    --out-dir dist \
    --no-modules \
    --no-typescript
