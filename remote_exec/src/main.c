#define _GNU_SOURCE
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "./remote_call.h"

void handler(int code);
void handler(int code)
{
    printf("SIGINT %d\n", code);
}

int main(int argc, char *argv[])
{
    if (argc < 2)
    {
        printf("Usage: %s <pid>\n", argv[0]);
        exit(1);
    }
    struct sigaction action = {0};
    action.sa_handler = handler;
    sigaction(SIGINT, &action, NULL);

    int target_pid = atoi(argv[1]);

    attach(target_pid);
    int child_pid = remote_fork(target_pid);
    detach(target_pid);

    attach(child_pid);
    Heap h = create_heap(child_pid, 0x1000);

    char *file_name = "/usr/bin/journalctl";
    void *file_name_buf = remote_malloc(&h, strlen(file_name) + 1);
    remote_memcpy(child_pid, file_name_buf, file_name, strlen(file_name) + 1);

    char *arg_1 = "-f";
    void *arg_1_buf = remote_malloc(&h, strlen(arg_1) + 1);
    remote_memcpy(child_pid, arg_1_buf, arg_1, strlen(arg_1) + 1);

    char *remote_argv[] = {file_name_buf, arg_1_buf, NULL};
    void *argv_buf = remote_malloc(&h, sizeof(remote_argv));
    remote_memcpy(child_pid, argv_buf, &remote_argv, sizeof(remote_argv));

    char *remote_envp[] = {NULL};
    void *envp_buf = remote_malloc(&h, sizeof(remote_envp));
    remote_memcpy(child_pid, envp_buf, &remote_envp, sizeof(remote_envp));

    remote_execv(child_pid, file_name_buf, argv_buf, envp_buf);
    detach(child_pid);
}

// load_args(int child_pid, char *argv, Heap *h)
// {
//     char *arg = argv;
//     while (arg)
//     {
//         void *arg_buf = remote_malloc(h, strlen(arg) + 1);
//         remote_memcpy(child_pid, arg_buf, arg, strlen(arg) + 1);
//         arg = argv++;
//     }
// }