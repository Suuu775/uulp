#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <stdlib.h>
#include <fcntl.h>

int main()
{
    int i;
    if (fork() != 0)
        exit(0);
    for (i = 1; i <= 10; i++)
    {
        printf("still here..In");
        sleep(i);
    }
    return 0;
}