
#include <internal.h>
#include <sys/timerfd.h>
#include <stdio.h>
#include <unistd.h>

int create_interval(int seconds)
{
    int timer_fd;
    if ((timer_fd = timerfd_create(CLOCK_MONOTONIC, TFD_CLOEXEC)) == -1)
    {
        perror("TIMERFD_CREATE");
    };
    const struct itimerspec timer_setting = {
        .it_interval.tv_sec = seconds,
        .it_interval.tv_nsec = 0,
        .it_value.tv_sec = seconds,
        .it_value.tv_nsec = 0};
    if (timerfd_settime(timer_fd, 0, &timer_setting, NULL) == -1)
    {
        perror("TIMERFD_SETTIME");
    };
    return timer_fd;
}

int read_interval(int timer_fd)
{
    uint64_t skipped;
    read(timer_fd, &skipped, sizeof(uint64_t));
    return skipped;
}
