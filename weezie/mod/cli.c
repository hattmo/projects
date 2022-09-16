#define MODULE_UUID 99

#include <weezie.h>
#include <stdio.h>
#include <unistd.h>
#include <sys/epoll.h>
#include <string.h>

char *prompt = ">> ";

EVENT_HANDLER(evt_handler,void)
{
    char *command[100] = {0};
    read(fd, command, 99);
    write(STDOUT_FILENO, prompt, strlen(prompt));
    write(STDOUT_FILENO,command,strlen(command));
}

FUNCTION_HANDLER(fn)
{
}

MODLULE_LOAD(load)
{
    add_event_handler(STDIN_FILENO, EPOLLIN, evt_handler, NULL);
    write(STDOUT_FILENO, prompt, strlen(prompt));
}

MODULE_INIT()
{
    register_module(load, fn);
}
