
#include <internal.h>
#include <sys/epoll.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

struct handler_entry
{
    int fd;
    Event_Handler handler;
    void *data;
};

static int epoll_fd = -1;
static int num_entries = 0;
static struct handler_entry *handler_entries = NULL;

static int add_handler_entry(int fd, Event_Handler handler, void *data)
{
    for (int i = 0; i < num_entries; i++)
    {
        if (handler_entries[i].fd == -1)
        {
            handler_entries[i] = (struct handler_entry){
                .fd = fd,
                .handler = handler,
                .data = data};
            return i;
        }
    }
    handler_entries = reallocarray(handler_entries, num_entries + 1, sizeof(struct handler_entry));
    handler_entries[num_entries] = (struct handler_entry){
        .fd = fd,
        .handler = handler,
        .data = data};
    int out = num_entries;
    num_entries++;
    return out;
}

int add_event_handler(int fd, int events, Event_Handler handler, void *data)
{
    struct epoll_event event = {0};
    event.events = events;
    event.data.u32 = add_handler_entry(fd, handler, data);
    if (epoll_ctl(epoll_fd, EPOLL_CTL_ADD, fd, &event) == -1)
    {
        return -1;
    };
    return event.data.u32;
}

void *remove_event_handler(int index)
{
    void *out = handler_entries[index].data;
    epoll_ctl(epoll_fd, EPOLL_CTL_DEL, handler_entries[index].fd, NULL);
    memset((handler_entries + index), 0, sizeof(struct handler_entry));
    handler_entries[index].fd = -1;
    return out;
}

int init_event_loop(void)
{
    if ((epoll_fd = epoll_create1(EPOLL_CLOEXEC)) == -1)
    {
        return FALSE;
    };
    return TRUE;
}

int start_event_loop(void)
{
    struct epoll_event ready_event;
    struct handler_entry ready_entry;
    while (1)
    {
        if (epoll_wait(epoll_fd, &ready_event, 1, -1) == -1)
        {
            return FALSE;
        }
        else
        {
            ready_entry = handler_entries[ready_event.data.u32];
            ready_entry.handler(ready_event.data.u32, ready_entry.fd, ready_entry.data);
        }
    }
}
