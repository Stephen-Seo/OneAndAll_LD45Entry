#ifndef LD45_ONE_AND_ALL_LIB_H_
#define LD45_ONE_AND_ALL_LIB_H_

extern void *ld45_initialize();

extern void ld45_iterate(void *context);

extern void ld45_save_async(void *data, int length);
extern void ld45_load_async(void *usr);
extern void ld45_load_rust_handler(void *usr, void *data, int len);

#endif
