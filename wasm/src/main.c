#include <emscripten.h>
#include <emscripten/html5.h>

#include <ld45_lib.h>

#include <stdio.h>

void main_loop(void *ud) {
    ld45_iterate(ud);
}

int main(void) {
    void *ld45_context = ld45_initialize();

    emscripten_set_main_loop_arg(main_loop, ld45_context, 0, 1);

    return 0;
}
