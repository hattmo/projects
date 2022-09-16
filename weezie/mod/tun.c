#define MODULE_UUID 4
#include <weezie.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <linux/if.h>
#include <linux/if_tun.h>
#include <sys/ioctl.h>
#include <sys/epoll.h>

static int tunnel_fd;
char packet_buffer[1600];
EVENT_HANDLER(evt_handler,void)
{
    ssize_t bytes_read = read(fd, packet_buffer, sizeof(packet_buffer));
    printf("Bytes Read {%ld}\n", bytes_read);
}

FUNCTION_HANDLER(fn)
{
}

MODLULE_LOAD(load)
{
    struct ifreq ifr = {0};
    if ((tunnel_fd = open("/dev/net/tun", O_RDWR | O_CLOEXEC)) == -1)
    {
        return;
    }
    ifr.ifr_flags = IFF_TUN;
    strncpy(ifr.ifr_name, "tun13", IFNAMSIZ);
    if (ioctl(tunnel_fd, TUNSETIFF, &ifr))
    {
        close(tunnel_fd);
        return;
    }
    add_event_handler(tunnel_fd, EPOLLIN, evt_handler, NULL);
}

MODULE_INIT()
{
    register_module(load, fn);
}
