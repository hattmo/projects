#include <sys/ptrace.h>
#include <sys/wait.h>
#include <stdlib.h>
#include <stdio.h>
#include <sys/user.h>
#include <inttypes.h>
#include <string.h>

int inject(pid_t target_pid);
int main(int argc, char **argv)
{
    if (argc < 2)
    {

        printf("%s", "invlaid args");
        return 1;
    }
    int target_pid = atoi(argv[1]);
    inject(target_pid);
}

int inject(pid_t target_pid)
{

    ptrace(PTRACE_ATTACH, target_pid);
    waitpid(target_pid, 0, 0);
    struct user_regs_struct old_regs;
    struct user_regs_struct new_regs;
    ptrace(PTRACE_GETREGS, target_pid, NULL, &old_regs);
    new_regs = old_regs;
    new_regs.rax = 0x000000000000003c;
    new_regs.rdi = 0x0000000000000007;
    uint8_t syscall[] = {0x0f, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};

    ptrace(PTRACE_POKETEXT, target_pid, new_regs.rip, *(uint64_t *)syscall);
    ptrace(PTRACE_SETREGS, target_pid, NULL, &new_regs);
    ptrace(PTRACE_DETACH, target_pid, NULL, NULL);
    printf("0x%016llx\n", new_regs.rip);
    return 0;
}