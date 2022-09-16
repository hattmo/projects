#define MODULE_UUID ~0U
#include "unity.h"
#include "weezie.h"
#include "internal.h"
#include <string.h>
void setUp(void)
{
    // set stuff up here
}

void tearDown(void)
{
    // clean stuff up here
}

void test_packet_list(void)
{
    // packet_list_t *packet1 = make_packet(20, 34, 7, 1234, 2345, 63456, 123, 2345, strlen("foo"), "foo");
    // packet_list_t *packet2 = make_packet(3456, 34, 7, 1234, 2345, 432, 123, 234, strlen("foo"), "foo");
    // packet_list_t *packet3 = make_packet(3456, 34, 6, 1234, 2345, 432, 123, 234, strlen("foo"), "foo");
    // packet1->next = packet2;
    // packet2->next = packet3;
    // add_packets(packet1);
    // print_packet_store();
    // packet_list_t *out = get_packets(7);
    // print_packet_store();
    // free_packets(out);
    // out = get_packets(6);
    // print_packet_store();
    // free_packets(out);
}

void test_conversions(void)
{
    char buf[] = {0, 0, 1, 10};
    uint32_t out;
    buftoint32(buf, &out);
    printf("%d\n", out);

    uint32_t in = 5;
    int32tobuf(&in, buf);
    printf("0x%.2hhx 0x%.2hhx 0x%.2hhx 0x%.2hhx\n", buf[0], buf[1], buf[2], buf[3]);
}

int main()
{
    UNITY_BEGIN();
    //   RUN_TEST(test_packet_list);
      RUN_TEST(test_conversions);
    return UNITY_END();
}
