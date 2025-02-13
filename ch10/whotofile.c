#include <wait.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <unistd.h>
#include<sys/stat.h>
#include <sys/types.h>

int main()
{
    int pid;
    int fd;

    printf("About to run who into a file\n");

    /* create a new process or quit */
    if ((pid = fork()) == -1)
    {
        perror("fork");
        exit(1);
    }
    /* child does the work */
    if (pid == 0)
    {
        close(1);
        fd = open("userlist",O_RDWR|O_CREAT|O_APPEND,S_IRUSR|S_IWUSR|S_IXUSR);
        execlp("who", "who", NULL);
        perror("execlp");
        exit(1);
    }
    /* parent waits then reports */
    if (pid != 0)
    {
        wait(NULL);
        printf("Done running who.  results in userlist\n");
    }
}
