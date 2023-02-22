CC = source "${HOME}/git/emsdk/emsdk_env.sh" && emcc

all: ld45.html

ld45.html: src/main.c ../target/wasm32-unknown-emscripten/release/libld45_lib.a
	${CC} -o ld45.html -s USE_GLFW=3 -Iinclude \
		-Llib -lraylib \
		-L../target/wasm32-unknown-emscripten/debug -lld45_lib \
		-sSAFE_HEAP=1 \
		-sEXPORTED_RUNTIME_METHODS=ccall,cwrap \
		--preload-file ../static src/main.c
	ln -sf ld45.html index.html

../target/wasm32-unknown-emscripten/debug/libld45_lib.a: ../src/lib.rs
	cd ..; source "${HOME}/git/emsdk/emsdk_env.sh"; cargo build --lib --target wasm32-unknown-emscripten

.PHONY: clean

clean:
	rm -f ld45.html
	rm -f ld45.js
	rm -f ld45.wasm
	rm -f ld45.data
	rm -f index.html
