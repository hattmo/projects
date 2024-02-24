#include <sys/socket.h>
#include <stdio.h>
int main(int argc, char *argv[])
{
    int fds[2];
    socketpair(AF_UNIX, SOCK_DGRAM, 0, fds);
    struct msghdr msg = {0};
    struct iovec io[1];
    struct cmsghdr *cmsg = CMSG_FIRSTHDR(&msg);

    io[0].iov_base = "hello";
    io[0].iov_len = strlen(io[0].iov_base);
    msg.msg_iov = &io;
    msg.msg_iovlen = sizeof(io) / sizeof(io[0]);
    msg.msg_controllen = 255;

    sendmsg(fds[0], &msg, 0);
}

static void send_fd(int socket, int fd) // send fd by socket
{
    struct msghdr msg = {0};
    char buf[CMSG_SPACE(sizeof(fd))];
    memset(buf, '\0', sizeof(buf));
    struct iovec io = {.iov_base = "ABC", .iov_len = 3};

    msg.msg_iov = &io;
    msg.msg_iovlen = 1;
    msg.msg_control = buf;
    msg.msg_controllen = sizeof(buf);

    struct cmsghdr *cmsg = CMSG_FIRSTHDR(&msg);
    cmsg->cmsg_level = SOL_SOCKET;
    cmsg->cmsg_type = SCM_RIGHTS;
    cmsg->cmsg_len = CMSG_LEN(sizeof(fd));

    *((int *)CMSG_DATA(cmsg)) = fd;

    msg.msg_controllen = CMSG_SPACE(sizeof(fd));

    if (sendmsg(socket, &msg, 0) < 0)
        err_syserr("Failed to send message\n");
}