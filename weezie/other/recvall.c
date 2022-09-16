#include <weezie.h>
#include <stdlib.h>
#include <sys/eventfd.h>
#include <sys/epoll.h>
struct recv_all_state
{
    int efd;
    char *buffer;
    size_t total_bytes;
    ssize_t *bytes_read;
};

struct recv_result
{
    ssize_t *bytes_read;
    char *buffer;
};

EVENT_HANDLER(recv_all_handler, struct recv_all_state)
{
    char *index = (data->buffer) + *data->bytes_read;
    size_t remaining = data->total_bytes - *data->bytes_read;
    ssize_t result = recv(fd, index, remaining, 0);
    if (result > 0)
    {
        *data->bytes_read += result;
        if (*data->bytes_read == data->total_bytes)
        {
            uint64_t wefd = 1;
            write(data->efd, &wefd, sizeof(uint64_t));
            free(data);
            remove_event_handler(index);
        }
    }
    else
    {
        *data->bytes_read = result;
        uint64_t wefd = 1;
        write(data->efd, &wefd, sizeof(uint64_t));
        free(data);
        remove_event_handler(index);
    }
}
void recv_all(int fd, size_t bytes, EVENT_HANDLE(cb, struct recv_result))
{
    int efd = eventfd(0, EFD_CLOEXEC | EFD_NONBLOCK);
    char *buffer = malloc(bytes);
    struct recv_all_state *state = malloc(sizeof(struct recv_all_state));
    struct recv_result *result = malloc(sizeof(struct recv_all_state));
    size_t *bytes_read = malloc(sizeof(size_t));
    *bytes_read = 0;
    *state = (struct recv_all_state){
        .buffer = buffer,
        .bytes_read = bytes_read,
        .total_bytes = bytes,
        .efd = efd};
    *result = (struct recv_result){
        .bytes_read = bytes_read,
        .buffer = buffer};
    add_event_handler(efd, EPOLLIN, cb, result);
    add_event_handler(fd, EPOLLIN, recv_all_handler, state);
}
