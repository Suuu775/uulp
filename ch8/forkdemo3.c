#include<stdio.h>
#include<stdlib.h>
#include<unistd.h>

int main(int argc, char const *argv[])
{
    int fork_rv;

    printf("Before my pid is %d\n",getpid());

    fork_rv = fork();

    if (fork_rv == -1)
    {
        perror("fork");
    }
    else if (fork_rv == 0)
    {
        printf("I am the chlid, my pid is %d\n",getpid());
    }
    else
    {
        printf("I am the parent,my chlid id is %d\n",fork_rv);
    }
    
    return 0;
}
