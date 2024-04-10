#define _GNU_SOURCE 1

#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <sched.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdlib.h>

#define NUM_THREADS 4
#define CHUNK_SIZE 4096
#define CHILD_STACK 4096
const int FLAGS = CLONE_VM | CLONE_FS | CLONE_FILES | CLONE_SIGHAND | CLONE_THREAD | CLONE_SYSVSEM | CLONE_SIGHAND;

struct Data
{
    unsigned long lock;
    char name[32];
    long max;
    long min;
    long sum;
    unsigned int count;
};

struct Context
{
    unsigned long front_lock;
    char front_chunk[CHUNK_SIZE];
    unsigned long back_lock;
    char back_chunk[CHUNK_SIZE];
};

struct Data data_store[UINT16_MAX] = {0};

void store_data(char *name, long val)
{
    uint16_t index = 0;
    index = (name[0] - 65) << 11;
    index |= (name[1] - 97) << 6;
    while (1)
    {
        struct Data *data = data_store + index;
        while (__atomic_exchange_n(&data->lock, 1, __ATOMIC_SEQ_CST))
        {
        }
        if (strcmp(data->name, name) == 0)
        {
            data->max = val > data->max ? val : data->max;
            data->min = val < data->min ? val : data->min;
            data->sum += val;
            data->count++;
        }
        else if (data->name[0] == 0)
        {
            strcpy(data->name, name);
            data->max = val;
            data->min = val;
            data->sum = val;
            data->count = 1;
        }
        else
        {
            __atomic_store_n(&data->lock, 0, __ATOMIC_SEQ_CST);
            index++;
            if (index == UINT16_MAX)
            {
                index = 0;
            }
            continue;
        }
        __atomic_store_n(&data->lock, 0, __ATOMIC_SEQ_CST);
        break;
    }
}

int process_data(char *chunk)
{
    int done = 0;
    while (1)
    {

        char *name = chunk;
        char *val_str = strchr(name, ';');
        *val_str = 0;
        val_str++;
        char *end_val = strchrnul(val_str, '\n');
        if (*end_val == 0)
        {
            done = 1;
        }
        *end_val = 0;
        *(end_val - 2) = *(end_val - 1);
        *(end_val - 1) = 0;
        long val = strtol(val_str, NULL, 10);
        store_data(name, val);
        end_val++;
        chunk = end_val;
        if (done)
        {
            break;
        }
    }
}

int task(void *arg)
{
    struct Context *ctx = (struct Context *)arg;
    while (1)
    {
        if (__atomic_load_n(&ctx->front_lock, __ATOMIC_SEQ_CST) == 1)
        {
            process_data(ctx->front_chunk);
            __atomic_store_n(&ctx->front_lock, 0, __ATOMIC_SEQ_CST);
        }
        if (__atomic_load_n(&ctx->back_lock, __ATOMIC_SEQ_CST) == 1)
        {
            process_data(ctx->back_chunk);
            __atomic_store_n(&ctx->back_lock, 0, __ATOMIC_SEQ_CST);
        }
    }
}

int read_into_buffer(int fd, char *leftover, uint8_t *leftover_size, char *chunk)
{

    memcpy(chunk, leftover, *leftover_size);
    ssize_t num_read = read(fd, chunk + *leftover_size, CHUNK_SIZE - *leftover_size);
    if (num_read == 0)
    {
        return 1;
    }
    num_read += *leftover_size;
    ssize_t index = num_read - 1;
    while (chunk[index] != '\n')
    {
        index--;
    }
    chunk[index] = 0;
    *leftover_size = (num_read - index) - 1;
    memcpy(leftover, chunk + index + 1, *leftover_size);
    return 0;
}

int main(int argc, char **argv)
{
    struct Context contexts[NUM_THREADS] = {0};

    uint8_t stack1[CHILD_STACK] = {0};
    clone(task, stack1 + CHILD_STACK, FLAGS, &contexts[0]);
    uint8_t stack2[CHILD_STACK] = {0};
    clone(task, stack2 + CHILD_STACK, FLAGS, &contexts[1]);
    uint8_t stack3[CHILD_STACK] = {0};
    clone(task, stack2 + CHILD_STACK, FLAGS, &contexts[2]);
    uint8_t stack4[CHILD_STACK] = {0};
    clone(task, stack2 + CHILD_STACK, FLAGS, &contexts[3]);

    char left_over[UINT8_MAX] = {0};
    uint8_t leftover_size = 0;
    int fd = open("other.txt", O_RDONLY);
    while (1)
    {
        for (int i = 0; i < NUM_THREADS; i++)
        {
            if (__atomic_load_n(&contexts[0].front_lock, __ATOMIC_SEQ_CST) == 0)
            {
                if (read_into_buffer(fd, left_over, &leftover_size, contexts[0].front_chunk))
                {
                    goto BREAK;
                };
                __atomic_store_n(&contexts[0].front_lock, 1, __ATOMIC_SEQ_CST);
            }
        }
    }
BREAK:
    for (int i = 0; i < NUM_THREADS; i++)
    {
        while (__atomic_load_n(&contexts[0].front_lock, __ATOMIC_SEQ_CST) == 1)
        {
        };
    }
    for (int i = 0; i < NUM_THREADS; i++)
    {
        while (__atomic_load_n(&contexts[0].back_lock, __ATOMIC_SEQ_CST) == 1)
        {
        };
    }

    for (uint16_t i = 0; i < UINT16_MAX; i++)
    {
        struct Data *data = data_store + i;
        if (data->name[0] != 0)
        {
            printf("name:%s max:%.1f min:%.1f avg:%.1f\n", data->name, data->max / 10.0, data->min / 10.0, (data->sum / 10.0) / (float)data->count);
        }
    }
}