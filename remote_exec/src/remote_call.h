#ifndef _REMOTE_CALL_H
#define _REMOTE_CALL_H 1
#include <sys/user.h>
#include <sys/mman.h>

static const unsigned long long syscall_opcode = 0x050F;

typedef struct
{
    struct user_regs_struct regs;
    long text;
} State;

typedef struct
{
    void *addr;
    size_t size;
} Entry;

typedef struct
{
    char *addr;
    size_t size;
    char *cursor;
} Heap;

int remote_fork(int pid);
void *remote_mmap(int pid, void *addr, size_t length, int prot, int flags, int fd, off_t offset);
int remote_munmap(int pid, void *addr, size_t length);
int remote_execv(int pid, const char *filename, const char *const *argv, const char *const *envp);

Heap create_heap(int pid, size_t size);
void destroy_heap(int pid, Heap heap);
void *remote_malloc(Heap* heap, size_t size);
void remote_memcpy(int pid, void *dst, void *src, size_t size);

void attach(int pid);
void detach(int pid);
#endif