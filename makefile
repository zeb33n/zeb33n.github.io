.PHONY: local
local:
	python3 -m http.server

.PHONY: rust
rust:
	cd zeblang_wasm && wasm-pack build --target web
	
