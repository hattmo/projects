#include <linux/futex.h>
#include <sys/syscall.h>
#include <inttypes.h>
#include <stdlib.h>

void take_mutex(uint32_t *mutex)
{
    while (__atomic_exchange_n(mutex, 1, __ATOMIC_SEQ_CST) == 1)
    {
        syscall(SYS_futex, mutex, FUTEX_WAIT, 1, NULL);
    }
}

void release_mutex(uint32_t *mutex)
{
    __atomic_store_n(mutex, 0, __ATOMIC_SEQ_CST);
    syscall(SYS_futex, mutex, FUTEX_WAKE, 1, NULL);
}
