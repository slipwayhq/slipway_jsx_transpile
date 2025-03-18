cargo build --target wasm32-wasip2 --release
rm -rf components
mkdir -p components/component
cp target/wasm32-wasip2/release/slipway_jsx_transform.wasm components/component/run.wasm
cp slipway_component.json components/component/slipway_component.json
slipway package components/component