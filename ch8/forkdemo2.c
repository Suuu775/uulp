#include<stdio.h>
#include<stdlib.h>
#include<unistd.h>

int main(int argc, char const *argv[])
{
    printf("my pid is %d\n",getpid());

    fork();
    fork();
    fork();

    printf("my pid is %d\n",getpid());
    return 0;
}