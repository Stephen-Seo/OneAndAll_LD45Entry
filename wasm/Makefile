CC = source "${HOME}/git/emsdk/emsdk_env.sh" && emcc

all: ld45.html

ld45.html: src/main.c ../target/wasm32-unknown-emscripten/release/libld45_lib.a
	${CC} -o ld45.html -s USE_GLFW=3 -Iinclude \
		-Llib -lraylib \
		-L../target/wasm32-unknown-emscripten/release -lld45_lib \
		-sTOTAL_MEMORY=1024MB \
		-sALLOW_MEMORY_GROWTH=1 \
		-O2 \
-fsanitize=address \
		-sEXPORTED_FUNCTIONS="['_malloc', '_main']" \
		-sEXPORTED_RUNTIME_METHODS=ccall,cwrap \
		--preload-file ../static src/main.c
	ln -sf ld45.html index.html

#-fsanitize=address \
#-sWARN_UNALIGNED=1 \

../target/wasm32-unknown-emscripten/release/libld45_lib.a: ../src/lib.rs
	cd ..; source "${HOME}/git/emsdk/emsdk_env.sh"; cargo build --lib --release --target wasm32-unknown-emscripten

.PHONY: clean

clean:
	rm -f ld45.html
	rm -f ld45.js
	rm -f ld45.wasm
	rm -f ld45.data
	rm -f index.html
	cd ..; cargo clean
