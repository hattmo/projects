#include <internal.h>
#include <stdio.h>

static uint32_t node_id;

uint32_t get_node_id(void)
{
    return node_id;
}

void init_node_id(void)
{
    node_id = get_random_int();
    printf("node_id: 0x%.8x\n", node_id);
}

#ifndef TEST_MODE
int main(void)
{
    init_event_loop();
    init_modules();
    start_event_loop();
}
#endif
