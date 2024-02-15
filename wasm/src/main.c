#include <emscripten.h>
#include <emscripten/html5.h>

#include <ld45_lib.h>

#include <stdio.h>

void ld45_saved_result_ok(void *usr) {
    puts("Save OK");
}

void ld45_saved_result_err(void *usr) {
    puts("Save ERR");
}

void ld45_load_result_ok(void *usr, void *data, int len) {
    ld45_load_rust_handler(usr, data, len);
}

void ld45_load_result_err(void *usr) {
    ld45_load_rust_handler(usr, NULL, 0);
}

void ld45_save_async(void *data, int length) {
    emscripten_idb_async_store("ld45_oneandall_db",
                               "savedata",
                               data,
                               length,
                               NULL,
                               ld45_saved_result_ok,
                               ld45_saved_result_err);
}

void ld45_load_async(void *usr) {
    emscripten_idb_async_load("ld45_oneandall_db",
                              "savedata",
                              usr,
                              ld45_load_result_ok,
                              ld45_load_result_err);
}

void main_loop(void *ud) {
    ld45_iterate(ud);
}

int main(void) {
    void *ld45_context = ld45_initialize();

    emscripten_set_main_loop_arg(main_loop, ld45_context, 0, 1);

    return 0;
}
