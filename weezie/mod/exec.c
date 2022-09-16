#include <unistd.h>
#include <stdlib.h>
int forking_exec(char *path, char **args, int *s_stdin, int *s_stdout, int *s_stderr)
{

    int stdin_p[2];
    int stdout_p[2];
    int stderr_p[2];
    pipe(stdin_p);
    pipe(stdout_p);
    pipe(stderr_p);
    int pid = fork();
    if (pid < 0)
    {
        close(stdin_p[0]);
        close(stdin_p[1]);
        close(stdout_p[0]);
        close(stdout_p[1]);
        close(stderr_p[0]);
        close(stderr_p[1]);
        return pid;
    }
    if (pid)
    {
        close(stdin_p[0]);
        close(stdout_p[1]);
        close(stderr_p[1]);
        *s_stdin = stdin_p[1];
        *s_stdout = stdout_p[0];
        *s_stderr = stderr_p[0];
        return pid;
    }
    else
    {
        close(stdin_p[1]);
        close(stdout_p[0]);
        close(stderr_p[0]);
        dup2(stdin_p[0], STDIN_FILENO);
        dup2(stdout_p[1], STDOUT_FILENO);
        dup2(stderr_p[1], STDERR_FILENO);
        execv(path, args);
        exit(0);
    }
}
