#include <internal.h>
#include <arpa/inet.h>
#include <sys/random.h>
#include <stdio.h>

void buftoint16(char *in, uint16_t *out)
{
    uint32_t tmp = 0;
    for (int i = 1; i >= 0; i--)
    {
        tmp <<= 8;
        tmp += (uint8_t)in[i];
    }
    *out = ntohs(tmp);
}

void int16tobuf(uint16_t *in, char *out)
{
    uint16_t tmp = *in;
    for (int i = 1; i >= 0; i--)
    {
        out[i] = (uint8_t)tmp;
        tmp >>= 8;
    }
}

void buftoint32(char *in, uint32_t *out)
{
    uint32_t tmp = 0;
    for (int i = 3; i >= 0; i--)
    {
        tmp <<= 8;
        tmp += (uint8_t)in[i];
    }
    *out = ntohl(tmp);
}

void int32tobuf(uint32_t *in, char *out)
{
    uint32_t tmp = *in;
    for (int i = 3; i >= 0; i--)
    {
        out[i] = (uint8_t)tmp;
        tmp >>= 8;
    }
}

uint32_t get_random_int(void)
{
    uint32_t out;
    getrandom(&out, sizeof(out), 0);
    return out;
}

void print_bytes(char *buf, int size)
{
    for (int i = 0; i < size; i++)
    {
        if (i % 16 == 0)
        {
            printf("\n");
        }
        printf("{%2.2x}", (uint8_t)buf[i]);
    }
    printf("\n");
}
