cp ../target/wasm32-unknown-emscripten/release/client.js static/
cp ../target/wasm32-unknown-emscripten/release/client.wasm static/
go run server.go
