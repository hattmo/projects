#ifndef WEEZIE_H
#define WEEZIE_H
#include "h.h"

#ifndef MODULE_UUID
#error MODULE_UUID not defined
#endif

#define EVENT_HANDLE(name,type) void (*name)(int,int,type*)

#define MODLULE_LOAD(fn) static void fn(void)
#define EVENT_HANDLER(fn,type) static void fn(int index, int fd, type *data)
#define FUNCTION_HANDLER(fn) static void fn(struct packet_list_t *packet)

#define MODULE_INIT_ID(id) __attribute__((constructor)) void init##_##id(void)
#define MODULE_INIT_TMP(id) MODULE_INIT_ID(id)
#define MODULE_INIT() MODULE_INIT_TMP(MODULE_UUID)

#define register_module(load, fn) register_module(MODULE_UUID, load, fn);

#endif
