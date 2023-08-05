#include <unistd.h>
#include <stdio.h>
#include <string.h>

int main(int argc, char **argv)
{
    int fds[2];
    pipe(fds);
    int read_fd = fds[0];
    int write_fd = fds[1];
    char *msg = "hello";
    write(read_fd, msg, strlen(msg));
}