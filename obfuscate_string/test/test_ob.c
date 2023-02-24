
#include <stdlib.h>

char blob1[] = {0x2f,0x26,0xb7};
char blob2[] = {0x1b,0x14,0xb7};
char *table[] = {0};

static char *lookup(int index, int start, int end)
{
    if (table[index] == 0)
    {
        char *target = malloc(end - start);
        table[index] = target;
        for (int i = start; i < end; i++)
        {
            *target = blob1[i] ^ blob2[i];
            target++;
        }
    }
    return table[index];
}

#include <stdio.h>

int main(int argc, char **argv)
{
    printf(lookup(0,0,9));
}