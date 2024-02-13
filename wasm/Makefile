EM_ENV = "${HOME}/git/emsdk/emsdk_env.sh"
CC = source ${EM_ENV} && emcc

all: ld45.html

ld45.html: src/main.c ../target/wasm32-unknown-emscripten/release/libld45_lib.a
	${CC} -o ld45.html -s USE_GLFW=3 -Iinclude \
		-Llib -lraylib \
		-L../target/wasm32-unknown-emscripten/release -lld45_lib \
		-sALLOW_MEMORY_GROWTH \
		-s INITIAL_MEMORY=64MB \
		-s TOTAL_STACK=32MB \
		-O2 \
		-sEXPORTED_FUNCTIONS="['_malloc', '_main']" \
		-sEXPORTED_RUNTIME_METHODS=ccall,cwrap \
		--preload-file ../static src/main.c
	ln -sf ld45.html index.html

#-fsanitize=address \
#-sWARN_UNALIGNED=1 \

../target/wasm32-unknown-emscripten/release/libld45_lib.a: ../src/lib.rs
	cd .. && source ${EM_ENV} && cargo build --lib --release --target wasm32-unknown-emscripten
#	cd .. && source ${EM_ENV} && cargo rustc --lib --release --target wasm32-unknown-emscripten -- -C link-args=-Wl,-zstack-size=8388608

.PHONY: clean

clean:
	rm -f ld45.html
	rm -f ld45.js
	rm -f ld45.wasm
	rm -f ld45.data
	rm -f index.html
	cd ..; cargo clean
