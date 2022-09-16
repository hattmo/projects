#include <internal.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <sys/epoll.h>
#include <string.h>
#include <unistd.h>
#include <array.h>
struct packet_store_entry
{
    uint32_t destination;
    time_t last_time;
    struct packet_list_head *list;
};

ARRAY_HEAD(packet_store, struct packet_store_entry);
static struct packet_store store = {0};

void clear_stale_packets(void)
{
    time_t cur_time;
    time(&cur_time);
    struct packet_store_entry *item;
    ARRAY_FOREACH(item, store)
    {
        if (cur_time - item->last_time > MAX_PACKET_CACHE_LIFE)
        {
            free_packet_list(item->list);
            memset(item, 0, sizeof(struct packet_store_entry));
        }
    }
}

void add_packets(struct packet_list_head *input_list)
{
    time_t cur_time;
    time(&cur_time);

    struct packet_list_t *new_packet;
    struct packet_store_entry *store_entry;
    CIRCLEQ_POP_ITEM(new_packet, input_list, entries);
    while (new_packet != NULL)
    {

        int found = FALSE;
        ARRAY_FOREACH(store_entry, store)
        {
            if (new_packet->data.to_node == store_entry->destination)
            {
                CIRCLEQ_INSERT_TAIL(store_entry->list, new_packet, entries);
                found = TRUE;
                break;
            }
        }
        if (found)
            continue;

        ARRAY_FOREACH(store_entry, store)
        {
            if (store_entry->destination == 0)
            {
                store_entry->last_time = cur_time;
                store_entry->destination = new_packet->data.to_node;
                store_entry->list = malloc(sizeof(struct packet_list_head));
                CIRCLEQ_INIT(store_entry->list);
                CIRCLEQ_INSERT_TAIL(store_entry->list, new_packet, entries);
                found = TRUE;
                break;
            }
        }
        if (found)
            continue;

        ARRAY_EXPAND(store, 1, struct packet_store_entry);
        ARRAY_LAST_ITEM(store) = (struct packet_store_entry){
            .destination = new_packet->data.to_node,
            .list = malloc(sizeof(struct packet_list_head)),
            .last_time = cur_time};
        CIRCLEQ_INIT(ARRAY_LAST_ITEM(store).list);
        CIRCLEQ_INSERT_TAIL(ARRAY_LAST_ITEM(store).list, new_packet, entries);

        CIRCLEQ_POP_ITEM(new_packet, input_list, entries);
    }
    struct packet_list_head *packets_for_self = malloc(sizeof(struct packet_list_head));
    CIRCLEQ_INIT(packets_for_self);
    get_packets(get_node_id(), packets_for_self);

    CIRCLEQ_FOREACH(new_packet, packets_for_self, entries)
    {
        call_module(new_packet->data.to_module, new_packet);
    }
    free_packet_list(packets_for_self);
    free_packet_list(input_list);
}

void get_packets(uint32_t destination, struct packet_list_head *out_list)
{
    struct packet_store_entry *store_entry;
    ARRAY_FOREACH(store_entry, store)
    {
        if (store_entry->destination != 0 && store_entry->destination == destination)
        {
            CIRCLEQ_JOIN(out_list, store_entry->list, entries);
            time_t cur_time;
            time(&cur_time);
            store_entry->last_time = cur_time;
            return;
        }
    }
    return;
}

void get_packets_for_via(uint32_t via, uint16_t module_id, struct packet_list_head *out_list)
{
    int state = 0;
    uint32_t dst;
    while ((dst = get_dst_from_via(via, module_id, &state)))
    {
        get_packets(dst, out_list);
    }
}

struct packet_list_t *make_packet(uint32_t to_node,
                                  uint16_t from_module,
                                  uint16_t to_module,
                                  uint8_t function_id,
                                  uint32_t stream_id,
                                  uint32_t sequence,
                                  uint32_t payload_size,
                                  char *payload)
{
    struct packet_list_t *out = calloc(1, sizeof(struct packet_list_t));
    out->data = (packet_t){
        .sender_node = get_node_id(),
        .from_node = get_node_id(),
        .to_node = to_node,
        .from_module = from_module,
        .to_module = to_module,
        .function_id = function_id,
        .hops = 0,
        .stream_id = stream_id,
        .sequence = sequence,
        .payload_size = payload_size};
    if (payload_size)
    {
        out->data.payload = calloc(1, payload_size);
        memcpy(out->data.payload, payload, payload_size);
    }
    else
    {
        out->data.payload = NULL;
    }
    return out;
}

struct packet_list_t *make_null_packet(uint32_t from_module)
{
    return make_packet(0, from_module, 1, 1, get_random_int(), 1, 0, NULL);
}

void free_packet_list(struct packet_list_head *input_list)
{
    struct packet_list_t *cur;
    CIRCLEQ_POP_ITEM(cur, input_list, entries);
    while (cur != NULL)
    {
        free(cur->data.payload);
        free(cur);
        CIRCLEQ_POP_ITEM(cur, input_list, entries);
    }
    free(input_list);
}

void serialize_packet_header(packet_t *packet, char *buf)
{
    int index = 0;
    int32tobuf(&(packet->sender_node), buf + index);
    index += 4;
    int32tobuf(&(packet->from_node), buf + index);
    index += 4;
    int32tobuf(&(packet->to_node), buf + index);
    index += 4;
    int16tobuf(&(packet->from_module), buf + index);
    index += 2;
    int16tobuf(&(packet->to_module), buf + index);
    index += 2;
    *(buf + index) = packet->function_id;
    index += 1;
    *(buf + index) = packet->hops;
    index += 1;
    int32tobuf(&(packet->stream_id), buf + index);
    index += 4;
    int32tobuf(&(packet->sequence), buf + index);
    index += 4;
    int32tobuf(&(packet->payload_size), buf + index);
}
void deserialize_packet_header(packet_t *packet, char *buf)
{
    int index = 0;
    buftoint32(buf + index, &(packet->sender_node));
    index += 4;
    buftoint32(buf + index, &(packet->from_node));
    index += 4;
    buftoint32(buf + index, &(packet->to_node));
    index += 4;
    buftoint16(buf + index, &(packet->from_module));
    index += 2;
    buftoint16(buf + index, &(packet->to_module));
    index += 2;
    packet->function_id = *(buf + index);
    index += 1;
    packet->hops = *(buf + index);
    index += 1;
    buftoint32(buf + index, &(packet->stream_id));
    index += 4;
    buftoint32(buf + index, &(packet->sequence));
    index += 4;
    buftoint32(buf + index, &(packet->payload_size));
}

char *serialize_packet_list(struct packet_list_head *list, uint32_t max_size, uint32_t *bytes_written)
{
    char *out = calloc(1, sizeof(uint32_t));
    *bytes_written = sizeof(uint32_t);
    struct packet_list_t *cur;

    CIRCLEQ_POP_ITEM(cur, list, entries);
    while (cur != NULL)
    {
        uint32_t packet_size = PACKET_HEADER_SZ + cur->data.payload_size;
        if (*bytes_written + packet_size > max_size)
        {
            break;
        }
        out = realloc(out, (*bytes_written) + packet_size);
        serialize_packet_header(&(cur->data), out + (*bytes_written));
        *bytes_written += PACKET_HEADER_SZ;
        memcpy(out + (*bytes_written), cur->data.payload, cur->data.payload_size);
        *bytes_written += cur->data.payload_size;
        free(cur->data.payload);
        free(cur);
        CIRCLEQ_POP_ITEM(cur, list, entries);
    }
    int32tobuf(bytes_written, out);
    return out;
}

struct packet_list_head *deserialize_packet_list(char *inbuf)
{
    uint32_t buf_size;
    buftoint32(inbuf, &buf_size);
    uint32_t bytes_read = sizeof(uint32_t);
    struct packet_list_head *out = malloc(sizeof(struct packet_list_head));
    CIRCLEQ_INIT(out);
    while (bytes_read + PACKET_HEADER_SZ <= buf_size)
    {
        struct packet_list_t *new_item;
        new_item = calloc(1, sizeof(struct packet_list_t));
        deserialize_packet_header(&(new_item->data), inbuf + bytes_read);
        bytes_read += PACKET_HEADER_SZ;
        if (bytes_read + new_item->data.payload_size > buf_size)
        {
            free(new_item);
            break;
        }
        new_item->data.payload = calloc(1, new_item->data.payload_size);
        memcpy(new_item->data.payload, inbuf + bytes_read, new_item->data.payload_size);
        CIRCLEQ_INSERT_TAIL(out, new_item, entries);
    }
    return out;
}

void print_packet_store(void)
{
    printf("--PACKET STORE--\n");

    struct packet_store_entry *store_entry;
    ARRAY_FOREACH(store_entry, store)
    {
        if (store_entry->destination != 0)
        {
            printf("\nDestination: %u Last Time: %lu\n", store_entry->destination, store_entry->last_time);
            printf("----------------------------------------------------------------------------------------------------\n");
            printf("|%10s|%10s|%10s|%10s|%10s|%10s|%10s|%10s|%10s|\n", "Sender", "From", "To", "From Mod", "To Mod", "Func", "Stream", "Sequence", "Size");
            printf("----------------------------------------------------------------------------------------------------\n");
            struct packet_list_t *cur;
            CIRCLEQ_FOREACH(cur, store_entry->list, entries)
            {
                packet_t data = cur->data;
                print_packet(data);
                printf("----------------------------------------------------------------------------------------------------\n");
            }
        }
    }
    printf("\n");
}

void print_packet(packet_t data)
{
    printf("|%10u|%10u|%10u|%10u|%10u|%10u|%10u|%10u|%10u|\n", data.sender_node, data.from_node, data.to_node, data.from_module, data.to_module, data.function_id, data.stream_id, data.sequence, data.payload_size);
}
