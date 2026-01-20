#include <bits/types/stack_t.h>
#include <signal.h>
#include <stddef.h>
#include <stdlib.h>
#include <ucontext.h>

ucontext_t *threads = NULL;
void **stacks = NULL;
int num_threads = 0;
int num_stacks = 0;
int thread_index = 0;

void *context_switcher(int sig, siginfo_t *info, void *data) {
  if (num_threads == 0) {
    return NULL;
  }
  thread_index += 1;
  if (thread_index >= num_threads) {
    thread_index = 0;
  }
  ucontext_t *ctx = data;

  return 0;
}

void wrapper(void (*entry)(void *arg), void *arg) {
  entry(arg);
  sigset_t set;
  sigemptyset(&set);
  sigaddset(&set, SIGINT);
  sigprocmask(SIG_BLOCK, &set, NULL);
  ucontext_t *last_ctx = threads + (num_threads - 1);
  num_threads -= 1;
  sigprocmask(SIG_UNBLOCK, &set, NULL);
  setcontext(last_ctx);
}

void spawn(void (*entry)(void *arg), void *arg) {
  sigset_t set;
  sigemptyset(&set);
  sigaddset(&set, SIGINT);
  sigprocmask(SIG_BLOCK, &set, NULL);

  num_threads += 1;
  threads = reallocarray(threads, num_threads, sizeof(ucontext_t));
  if (num_stacks < num_threads) {
    num_stacks += 1;
    stacks = reallocarray(stacks, num_stacks, sizeof(void *));
    stacks[num_stacks - 1] = calloc(SIGSTKSZ, sizeof(char));
  }
  
  void *stack_ptr = stacks + (num_threads - 1);
  stack_t new_stack = {.ss_size = SIGSTKSZ, .ss_sp = stack_ptr, .ss_flags = 0};

  ucontext_t *last_ctx = threads + (num_threads - 1);
  last_ctx->uc_link = NULL;
  last_ctx->uc_stack = new_stack;

  makecontext(last_ctx, (void (*)())wrapper, 2, entry, arg);

  sigprocmask(SIG_UNBLOCK, &set, NULL);
}
