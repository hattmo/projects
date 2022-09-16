#define MODULE_UUID 1
#include <weezie.h>
#include <internal.h>
#include <sys/epoll.h>
#include <sys/signalfd.h>
#include <stdlib.h>
#include <stdio.h>
#include <signal.h>
#include <unistd.h>
#include <string.h>

EVENT_HANDLER(clear_caches, void)
{
    read_interval(fd);
    clear_stale_packets();
    clear_stale_router_store();
}

EVENT_HANDLER(sighandler, void)
{
    struct signalfd_siginfo info;
    read(fd, &info, sizeof(struct signalfd_siginfo));
    switch (info.ssi_signo)
    {
    case SIGINT:
        char *msg = "SIGINT... Exiting\n";
        write(STDOUT_FILENO, msg, strlen(msg));
        exit(0);
    }
}

FUNCTION_HANDLER(fn)
{
}

MODLULE_LOAD(load)
{
    init_node_id();
    int timer = create_interval(60);
    add_event_handler(timer, EPOLLIN, clear_caches, NULL);

    sigset_t all_mask;
    sigfillset(&all_mask);
    sigdelset(&all_mask, SIGBUS);
    sigdelset(&all_mask, SIGFPE);
    sigdelset(&all_mask, SIGILL);
    sigdelset(&all_mask, SIGSEGV);
    sigprocmask(SIG_SETMASK, &all_mask, NULL);

    int sig = signalfd(-1, &all_mask, SFD_CLOEXEC | SFD_NONBLOCK);
    add_event_handler(sig, EPOLLIN, sighandler, NULL);
}

MODULE_INIT()
{
    register_module(load, fn);
}
