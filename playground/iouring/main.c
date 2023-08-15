#define _GNU_SOURCE
#include <liburing.h>

int main(int argc, char *argv[])
{
    struct io_uring ring;
    io_uring_setup(64, &ring);
}