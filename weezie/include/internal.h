#ifndef INTERNAL_H
#define INTERNAL_H
#include "h.h"

#define MAX_PACKET_CACHE_LIFE 3600
#define MAX_ROUTER_CACHE_LIFE 3600
#define PACKET_HEADER_SZ 30
// packets
void clear_stale_packets(void);
void print_packet_store(void);

// event_loop
int init_event_loop(void);
int start_event_loop(void);

// module_manager
void call_module(uint32_t module_id, struct packet_list_t *packets);
void init_modules(void);

// main
void init_node_id(void);

//router
uint32_t get_dst_from_via(uint32_t via, uint16_t module_id, int *state);
void clear_stale_router_store(void);
#endif
