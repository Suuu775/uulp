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
    int fd;
    int pid;
    char msg1[] = "Test 1 2 3..\n";
    char msg2[] = "Hello,hello\n";
    if ((fd = creat("tesefile", 0644)) == -1)
        return 0;
    if (write(fd, msg1, strlen(msg1)) == -1)
        return 0;
    if ((pid = fork()) == -1)
        return 0;
    if (write(fd, msg2, strlen(msg2)) == - 1)
        return 0;
    close(fd);
    return 1;
}