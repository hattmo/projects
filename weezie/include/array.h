#define ARRAY_HEAD(name, type) \
    struct name                \
    {                          \
        type *data;            \
        int size;              \
    }

#define ARRAY_FOREACH(var, head) for (var = head.data; var - head.data < head.size; var++)

#define ARRAY_EXPAND(head, n, type)                                       \
    do                                                                    \
    {                                                                     \
        head.data = reallocarray(head.data, head.size + n, sizeof(type)); \
        head.size += n;                                                   \
    } while (0)

#define ARRAY_LAST_ITEM(head) head.data[head.size - 1]

#define CIRCLEQ_POP_ITEM(var, head, field)                         \
    do                                                             \
    {                                                              \
        if (CIRCLEQ_EMPTY(head))                                   \
        {                                                          \
            var = NULL;                                            \
        }                                                          \
        else                                                       \
        {                                                          \
            var = (head)->cqh_first;                               \
            (head)->cqh_first = (head)->cqh_first->field.cqe_next; \
            (head)->cqh_first->field.cqe_prev = (void*)(head);            \
        }                                                          \
    } while (1)

#define CIRCLEQ_JOIN(to, from, field)                           \
    do                                                          \
    {                                                           \
        if (CIRCLEQ_EMPTY((to)))                                \
        {                                                       \
            (to)->cqh_first = (from)->cqh_first;                \
            (to)->cqh_first->field.cqe_prev = (void*)(to);             \
        }                                                       \
        else                                                    \
        {                                                       \
            (to)->cqh_last->field.cqe_next = (from)->cqh_first; \
            (from)->cqh_first->field.cqe_prev = (to)->cqh_last; \
        }                                                       \
        (to)->cqh_last = (from)->cqh_last;                      \
        (to)->cqh_last->field.cqe_next = (void*)(to);                  \
        CIRCLEQ_INIT(from);                                     \
    } while (0);
