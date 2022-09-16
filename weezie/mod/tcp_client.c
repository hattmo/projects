#define MODULE_UUID 2
#include <weezie.h>
#include <sys/epoll.h>
#include <stdio.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <sys/eventfd.h>
#include <netdb.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>

struct client_connection
{
    CIRCLEQ_ENTRY(client_connection)
    entries;
    char *host;
    char *port;
    int event_index;
    uint32_t connected_node;
};

CIRCLEQ_HEAD(client_connection_head, client_connection);
struct client_connection_head connections;

struct recv_buf
{
    char *buf;
    size_t size;
    uint32_t read;
    uint32_t *connected_node_ref;
};

EVENT_HANDLER(recieve_data_client, struct recv_buf)
{

    ssize_t bytes_read = recv(fd, data->buf + data->read, data->size - data->read, MSG_DONTWAIT);

    if (bytes_read <= 0)
    {
        printf("ERROR RECV");
        free(data->buf);
        free(data);
        close(fd);
        remove_event_handler(index);
        return;
    }

    data->read += bytes_read;

    if (data->size == sizeof(uint32_t) && data->read == sizeof(uint32_t))
    {
        uint32_t total_size;
        buftoint32(data->buf, &total_size);
        data->buf = realloc(data->buf, total_size);
        data->size = total_size;
        return;
    }

    if (data->size == data->read)
    {
        struct packet_list_head *packets = deserialize_packet_list(data->buf);
        if (!CIRCLEQ_EMPTY(packets))
        {
            *(data->connected_node_ref) = CIRCLEQ_FIRST(packets)->data.sender_node;
        }
        add_packets(packets);
        free_packet_list(packets);
        free(data->buf);
        free(data);
        close(fd);
        remove_event_handler(index);
    }
}

EVENT_HANDLER(call_upstream, struct client_connection)
{
    read_interval(fd);
    int sockfd;
    if ((sockfd = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0)) < 0)
    {
        return;
    }
    struct addrinfo *res = NULL;
    getaddrinfo(data->host, data->port, NULL, &res);
    int found = 0;
    for (struct addrinfo *i = res; i != NULL; i = i->ai_next)
    {
        if (!connect(sockfd, i->ai_addr, i->ai_addrlen))
        {
            found = 1;
            break;
        }
    }
    if (!found)
    {
        freeaddrinfo(res);
        close(sockfd);
        return;
    }

    struct packet_list_head *out_packets = calloc(1, sizeof(struct packet_list_head));
    CIRCLEQ_INIT(out_packets);
    if (data->connected_node == 0)
    {
        struct packet_list_t *null_packet = make_null_packet(MODULE_UUID);
        CIRCLEQ_INSERT_HEAD(out_packets, null_packet, entries);
    }
    else
    {
        get_packets_for_via(data->connected_node, MODULE_UUID, out_packets);
    }

    uint32_t buf_size;
    char *buf = serialize_packet_list(out_packets, 100, &buf_size);
    send(sockfd, buf, buf_size, 0);

    free(buf);
    free_packet_list(out_packets);
    freeaddrinfo(res);

    struct recv_buf *recvbuf = calloc(1, sizeof(struct recv_buf));
    recvbuf->buf = calloc(1, sizeof(uint32_t));
    recvbuf->size = 4;
    add_event_handler(sockfd, EPOLLIN, recieve_data_client, recvbuf);
}

void add_tcp_client_connection(uint32_t size, char *data)
{
    struct client_connection *new_conn = calloc(1, sizeof(struct client_connection));
    char *cur = data;
    uint32_t sleep_time;
    buftoint32(cur, &sleep_time);
    cur += 4;

    uint16_t host_size;
    buftoint16(cur, &host_size);
    cur += 2;
    new_conn->host = calloc(host_size + 1, sizeof(char));
    memcpy(new_conn->host, cur, host_size);
    cur += host_size;

    uint16_t port_size;
    buftoint16(cur, &port_size);
    cur += 2;
    new_conn->port = calloc(port_size + 1, sizeof(char));
    memcpy(new_conn->host, cur, port_size);
    cur += port_size;

    int timer = create_interval(sleep_time);
    new_conn->event_index = add_event_handler(timer, EPOLLIN, call_upstream, new_conn);
    CIRCLEQ_INSERT_HEAD(&connections, new_conn, entries);
}

FUNCTION_HANDLER(fn)
{
    packet_t data = packet->data;
    switch (data.function_id)
    {
    case 1:
        add_tcp_client_connection(data.payload_size, data.payload);
        break;
    // case 2:
    //     remove_tcp_client_connection(data.payload_size, data.payload);
    //     break;
    // case 3:
    //     query_tcp_client_connection(data.payload_size, data.payload);
    }
    free_packet_list(packet);
}

MODLULE_LOAD(load)
{
}

MODULE_INIT()
{
    CIRCLEQ_INIT(&connections);
    register_module(load, fn);
}
