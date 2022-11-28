cp ../target/wasm32-unknown-emscripten/debug/client.js static/
cp ../target/wasm32-unknown-emscripten/debug/client.wasm static/
go run server.go
