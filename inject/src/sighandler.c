#define _GNU_SOURCE

#include <unistd.h>
#include <stdio.h>
#include <signal.h>
#include <stddef.h>
#include <stdlib.h>
#include <time.h>
#include <ucontext.h>

ucontext_t *threads = NULL;
void **stacks = NULL;
int num_threads = 0;
int num_stacks = 0;
int thread_index = 0;

void context_switcher(int sig, siginfo_t *info, void *data) {
  printf("Called handler\n");
  if (num_threads == 0) {
    return;
  }
  thread_index += 1;
  if (thread_index >= num_threads) {
    thread_index = 0;
  }
  ucontext_t *running_thread = data;
  gregset_t* regs = &running_thread->uc_mcontext.gregs;
  printf("inturrupted:\n\tRSP: %p\n\tRIP: %p\n\tRBP: %p\n", (*regs)[REG_RSP], (*regs)[REG_RIP], (*regs)[REG_RBP]);
  ucontext_t paused_thread = threads[thread_index];
  threads[thread_index] = *running_thread;
  setcontext(&paused_thread);  // NO RETURN
  return;
}

void wrapper(void (*entry)(void *arg), void *arg) {
  entry(arg);
  sigset_t set;
  sigemptyset(&set);
  sigaddset(&set, SIGRTMIN);
  sigprocmask(SIG_BLOCK, &set, NULL);
  ucontext_t last_ctx = threads[num_threads - 1];
  num_threads -= 1;
  sigprocmask(SIG_UNBLOCK, &set, NULL);
  setcontext(&last_ctx);  // NO RETURN
}

void spawn(void (*entry)(void *arg), void *arg) {
  sigset_t set;
  sigemptyset(&set);
  sigaddset(&set, SIGRTMIN);
  sigprocmask(SIG_BLOCK, &set, NULL);

  num_threads += 1;
  threads = reallocarray(threads, num_threads, sizeof(ucontext_t));
  if (num_stacks < num_threads) {
    num_stacks += 1;
    stacks = reallocarray(stacks, num_stacks, sizeof(void *));
    stacks[num_stacks - 1] = calloc(SIGSTKSZ, sizeof(char));
  }

  void *stack_ptr = stacks[num_threads - 1];
  printf("Stack ptr: %p\n", stack_ptr);
  stack_t new_stack = {.ss_size = SIGSTKSZ, .ss_sp = stack_ptr, .ss_flags = 0};
  ucontext_t *last_ctx = threads + (num_threads - 1);
  getcontext(last_ctx);
  sigset_t new_thread_mask;
  sigemptyset(&new_thread_mask);
  last_ctx->uc_sigmask = new_thread_mask;
  last_ctx->uc_link = NULL;
  last_ctx->uc_stack = new_stack;
  makecontext(last_ctx, (void (*)())wrapper, 2, entry, arg);

  sigprocmask(SIG_UNBLOCK, &set, NULL);
}

int init() {
  struct sigaction act;
  act.sa_sigaction = context_switcher;
  act.sa_flags = SA_SIGINFO;
  sigaction(SIGRTMIN, &act, NULL);

  struct sigevent evp;
  evp.sigev_signo = SIGRTMIN;
  evp.sigev_notify = SIGEV_SIGNAL;
  evp.sigev_value.sival_ptr = NULL;

  struct itimerspec value;
  value.it_interval.tv_sec = 3;
  value.it_interval.tv_nsec = 0;
  value.it_value.tv_sec = 3;
  value.it_value.tv_nsec = 0;

  timer_t timerid;
  timer_create(CLOCK_MONOTONIC, &evp, &timerid);
  timer_settime(timerid, 0, &value, NULL);

  return 0;
}

void worker(void *arg) {
  while (1) {
    // printf("In Worker\n");
    // sleep(1);
  }
}

int main() {
  init();
  spawn(worker, NULL);
  while (1) {
    // printf("In Main\n");
    // sleep(1);
  }
  return 0;
}
