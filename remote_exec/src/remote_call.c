#include <string.h>
#include <sys/ptrace.h>
#include <sys/user.h>
#include <stdarg.h>
#include <stdint.h>
#include <sys/wait.h>
#include <sys/syscall.h>
#include <stdlib.h>

#include "./remote_call.h"

State prepare(int pid);
void restore(int pid, State state);
unsigned long long remote_syscall(int pid, State state, unsigned long long sys_call, int argc, ...);

void attach(int pid)
{
    ptrace(PTRACE_ATTACH, pid, NULL, NULL);
    int status;
    waitpid(pid, &status, 0);
}

void detach(int pid)
{
    ptrace(PTRACE_DETACH, pid, NULL, NULL);
}

State prepare(int pid)
{
    State out = {0};
    struct user_regs_struct backup_regs;
    ptrace(PTRACE_GETREGS, pid, NULL, &backup_regs);
    out.regs = backup_regs;
    long old_text = ptrace(PTRACE_PEEKTEXT, pid, backup_regs.rip, NULL);
    out.text = old_text;
    return out;
}

void restore(int pid, State state)
{
    ptrace(PTRACE_POKETEXT, pid, state.regs.rip, state.text);
    ptrace(PTRACE_SETREGS, pid, NULL, &state.regs);
}

unsigned long long remote_syscall(int pid, State state, unsigned long long sys_call, int argc, ...)
{

    struct user_regs_struct regs = state.regs;

    va_list argp;
    va_start(argp, argc);
    for (int i = 0; i < argc; i++)
    {
        unsigned long long arg = va_arg(argp, unsigned long long);
        switch (i)
        {
        case 0:
            regs.rdi = arg;
            break;
        case 1:
            regs.rsi = arg;
            break;
        case 2:
            regs.rdx = arg;
            break;
        case 3:
            regs.r10 = arg;
            break;
        case 4:
            regs.r8 = arg;
            break;
        case 5:
            regs.r9 = arg;
            break;
        default:
            return -1;
        }
    }
    va_end(argp);

    regs.rax = sys_call;
    ptrace(PTRACE_POKETEXT, pid, regs.rip, syscall_opcode);
    ptrace(PTRACE_SETREGS, pid, NULL, &regs);

    int status;
    ptrace(PTRACE_SYSCALL, pid, NULL, NULL);
    waitpid(pid, &status, 0);
    ptrace(PTRACE_SYSCALL, pid, NULL, NULL);
    waitpid(pid, &status, 0);
    ptrace(PTRACE_GETREGS, pid, NULL, &regs);
    return regs.rax;
}

Heap create_heap(int pid, size_t size)
{
    State state = prepare(pid);
    void *addr = remote_mmap(pid, NULL, size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    restore(pid, state);
    Heap heap = {.addr = addr, .size = size, .cursor = addr};
    return heap;
}

void destroy_heap(int pid, Heap heap)
{
    State state = prepare(pid);
    remote_munmap(pid, heap.addr, heap.size);
    restore(pid, state);
}

void *remote_malloc(Heap *heap, size_t size)
{
    if (heap->cursor + size > heap->addr + heap->size)
    {
        return NULL;
    }
    void *out = heap->cursor;
    heap->cursor += size;
    return out;
}

void remote_memcpy(int pid, void *dst, void *src, size_t size)
{
    size_t remainder = size % 8;
    size_t aligned_size = size - remainder;

    for (size_t i = 0; i < aligned_size; i += 8)
    {
        ptrace(PTRACE_POKETEXT, pid, (char *)dst + i, *(long long *)((char *)src + i));
    }
    if (remainder > 0)
    {
        long long saved = ptrace(PTRACE_PEEKTEXT, pid, (char *)dst + aligned_size, NULL);
        for (size_t i = 0; i < remainder; i++)
        {
            *(((char *)&saved) + i) = *((char *)src + aligned_size + i);
        }
        ptrace(PTRACE_POKETEXT, pid, (char *)dst + aligned_size, saved);
    }
}

int remote_fork(int pid)
{
    State state = prepare(pid);
    ptrace(PTRACE_SETOPTIONS, pid, NULL, PTRACE_O_TRACEFORK);
    remote_syscall(pid, state, SYS_fork, 0);
    long long ret;
    ptrace(PTRACE_GETEVENTMSG, pid, NULL, &ret);
    restore(pid, state);
    return ret;
}

void *remote_mmap(int pid, void *addr, size_t length, int prot, int flags, int fd, off_t offset)
{
    State state = prepare(pid);
    unsigned long long ret = remote_syscall(pid, state, SYS_mmap, 6, addr, length, prot, flags, fd, offset);
    restore(pid, state);
    return (void *)ret;
}

int remote_munmap(int pid, void *addr, size_t length)
{
    State state = prepare(pid);
    unsigned long long ret = remote_syscall(pid, state, SYS_munmap, 2, addr, length);
    restore(pid, state);
    return (int)ret;
}

int remote_execv(int pid, const char *filename, const char *const *argv, const char *const *envp)
{
    State state = prepare(pid);
    ptrace(PTRACE_SETOPTIONS, pid, NULL, PTRACE_O_TRACEEXEC);
    remote_syscall(pid, state, SYS_execve, 3, filename, argv, envp);
    long long ret;
    ptrace(PTRACE_GETEVENTMSG, pid, NULL, &ret);
    return (int)ret;
}