#include<stdio.h>
#include<signal.h>
#include<unistd.h>

int main(int argc, char const *argv[])
{
    signal(SIGINT,SIG_IGN);
    printf("you can't stop me!\n");
    while (1)
    {
        sleep(1);
        printf("jiejiejieje\n");
    }
    return 0;
}
