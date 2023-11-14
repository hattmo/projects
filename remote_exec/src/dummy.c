#include <unistd.h>
#include <stdio.h>
int main(int argc, char **argv)
{
    while (1)
    {
        pause();
        printf("I was interrupted\n");
    }
}