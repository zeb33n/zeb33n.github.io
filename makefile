.PHONY: local
local:
	python3 -m http.server

.PHONY: rust
rust:
	cd zeblang_wasm && wasm-pack build --target web
	
.PHONY: rust_u
rust_u:
	cd zeblang_wasm && cargo update && wasm-pack build --target web
	
