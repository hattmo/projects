#include <array.h>
#include <internal.h>
#include <stdlib.h>
#include <string.h>
struct router_store_entry_t
{
    uint32_t dst;
    uint32_t via;
    uint16_t module_id;
    uint8_t hops;
    time_t last_time;
};

ARRAY_HEAD(router_entries, struct router_store_entry_t)
routes;

void clear_stale_router_store(void)
{
    time_t cur_time;
    time(&cur_time);
    struct router_store_entry_t *entry;
    ARRAY_FOREACH(entry, routes)
    {
        if (cur_time - entry->last_time > MAX_ROUTER_CACHE_LIFE)
        {
            memset(entry, 0, sizeof(struct router_store_entry_t));
        }
    }
}

void add_router_store_entry(uint32_t dst,
                            uint32_t via,
                            uint16_t module_id,
                            uint8_t hops)
{
    struct router_store_entry_t *entry;
    ARRAY_FOREACH(entry, routes)
    {
        if (entry->dst == dst)
        {
            if (entry->via == via && entry->module_id == module_id && entry->hops == hops)
            {
                time_t cur_time;
                time(&cur_time);
                entry->last_time = cur_time;
            }
            else if (entry->hops > hops)
            {
                time_t cur_time;
                time(&cur_time);
                *entry = (struct router_store_entry_t){
                    .last_time = cur_time,
                    .via = via,
                    .module_id = module_id,
                    .hops = hops};
            }
            return;
        }
    }

    ARRAY_FOREACH(entry, routes)
    {
        time_t cur_time;
        time(&cur_time);
        if (entry->dst == 0)
        {
            *entry = (struct router_store_entry_t){
                .last_time = cur_time,
                .via = via,
                .module_id = module_id,
                .hops = hops};
            return;
        }
    }

    time_t cur_time;
    time(&cur_time);
    ARRAY_EXPAND(routes, 1, struct router_store_entry_t);
    ARRAY_LAST_ITEM(routes) = (struct router_store_entry_t){
        .dst = dst,
        .via = via,
        .module_id = module_id,
        .hops = hops,
        .last_time = cur_time};
}

void remove_router_entry_via(uint32_t via, uint16_t module_id)
{
    struct router_store_entry_t *entry;
    ARRAY_FOREACH(entry, routes)
    {
        if (entry->via == via && entry->module_id == module_id)
        {
            memset(entry, 0, sizeof(struct router_store_entry_t));
        }
    }
}

void remove_router_entry_dst(uint32_t dst)
{
    struct router_store_entry_t *entry;
    ARRAY_FOREACH(entry, routes)
    {
        if (entry->dst == dst)
        {
            memset(entry, 0, sizeof(struct router_store_entry_t));
            return;
        }
    }
}

uint32_t get_dst_from_via(uint32_t via, uint16_t module_id, int *state)
{
    struct router_store_entry_t entry;
    for (int i = *state; i < routes.size; i++)
    {
        entry = routes.data[i];
        if (entry.via == via && entry.module_id == module_id)
        {
            *state = i + 1;
            return entry.dst;
        }
    }
    *state = routes.size;
    return 0;
}

uint32_t get_connected_nodes(int *state)
{
    struct router_store_entry_t entry;
    for (int i = *state; i < routes.size; i++)
    {
        entry = routes.data[i];
        if (entry.hops == 1)
        {
            *state = i + 1;
            return entry.dst;
        }
    }
    *state = routes.size;
    return 0;
}
