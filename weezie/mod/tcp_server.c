#define MODULE_UUID 3

#include <weezie.h>
#include <sys/socket.h>
#include <netdb.h>
#include <sys/epoll.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>

EVENT_HANDLER(recieve_data_server, void)
{
    char *recv_buffer = calloc(1, sizeof(uint32_t));
    recv(fd, recv_buffer, 4, 0);
    uint32_t packets_size = 0;
    buftoint32(recv_buffer, &packets_size);
    recv_buffer = realloc(recv_buffer, packets_size);
    recv(fd, recv_buffer + 4, packets_size - 4, 0);
    struct packet_list_t *recv_packets = deserialize_packet_list(recv_buffer);

    write(STDOUT_FILENO, "Server recieve packet:\n", 23);
    print_packet(recv_packets->data);

    free(recv_buffer);
    free_packet_list(recv_packets);

    struct packet_list_t *send_packet = make_null_packet(MODULE_UUID);
    write(STDOUT_FILENO, "Server send packet:\n", 20);
    print_packet(send_packet->data);
    uint32_t buf_size;
    char *send_buf = serialize_packet_list(&send_packet, 100, &buf_size);
    send(fd, send_buf, buf_size, 0);

    free(send_buf);
    free_packet_list(send_packet);
    close(fd);
    remove_event_handler(index);
}

EVENT_HANDLER(accept_client, void)
{
    int client = accept4(fd, NULL, NULL, SOCK_CLOEXEC);
    add_event_handler(client, EPOLLIN, recieve_data_server, NULL);
}

FUNCTION_HANDLER(fn)
{
}

MODLULE_LOAD(load)
{
    int servsock = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0);
    struct sockaddr_in addr = {0};
    addr.sin_family = AF_INET;
    addr.sin_port = htons(8000);
    bind(servsock, &addr, sizeof(addr));
    listen(servsock, 200);
    add_event_handler(servsock, EPOLLIN, accept_client, NULL);
}

MODULE_INIT()
{
    register_module(load, fn);
}
