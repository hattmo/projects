#ifndef WEEZIE_H_H
#define WEEZIE_H_H
#define _GNU_SOURCE 1

#define TRUE 1
#define FALSE 0

#include <stdint.h>
#include <time.h>
#include <sys/queue.h>
typedef struct packet_t
{
    uint32_t sender_node;
    uint32_t from_node;
    uint32_t to_node;
    uint16_t from_module;
    uint16_t to_module;
    uint8_t function_id;
    uint8_t hops;
    uint32_t stream_id;
    uint32_t sequence;
    uint32_t payload_size;
    char *payload;
} packet_t;

struct packet_list_t
{
    CIRCLEQ_ENTRY(packet_list_t)
    entries;
    packet_t data;
};

CIRCLEQ_HEAD(packet_list_head, packet_list_t);

typedef void (*Event_Handler)(int index, int fd, void *data);
typedef void (*Function_Handler)(struct packet_list_t *packet);
typedef void (*Module_Load)(void);

// packets
void add_packets(struct packet_list_head *input_list);
void get_packets_for_via(uint32_t via,
                         uint16_t module_id,
                         struct packet_list_head *out_list);
void get_packets(uint32_t destination, struct packet_list_head *out_list);
void free_packet_list(struct packet_list_head *input_list);
struct packet_list_t *make_packet(uint32_t to_node,
                                  uint16_t from_module,
                                  uint16_t to_module,
                                  uint8_t function_id,
                                  uint32_t stream_id,
                                  uint32_t sequence,
                                  uint32_t payload_size,
                                  char *payload);
struct packet_list_t *make_null_packet(uint32_t from_module);
struct packet_list_head *deserialize_packet_list(char *inbuf);
char *serialize_packet_list(struct packet_list_head *list, uint32_t max_size, uint32_t *bytes_written);
void print_packet_store(void);
void print_packet(packet_t data);

// time_util
int create_interval(int seconds);
int read_interval(int timer_fd);

// event_handler
int add_event_handler(int fd, int events, Event_Handler handler, void *data);
void *remove_event_handler(int index);

// util
void buftoint16(char *in, uint16_t *out);
void int16tobuf(uint16_t *in, char *out);
void buftoint32(char *in, uint32_t *out);
void int32tobuf(uint32_t *in, char *out);
uint32_t get_random_int(void);
void print_bytes(char *buf, int size);

// module_manager
void register_module(uint32_t module_id, Module_Load mod_load, Function_Handler fn);

// main
uint32_t get_node_id(void);

// router
void add_router_store_entry(uint32_t dst,
                            uint32_t via,
                            uint16_t module_id,
                            uint8_t hops);
#endif
