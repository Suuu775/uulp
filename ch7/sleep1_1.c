#include<stdio.h>
#include<signal.h>
#include<unistd.h>
// #define SHHH

int main(int argc, char const *argv[])
{
    signal(SIGINT,SIG_IGN);
    void wakeup(int);
    signal(SIGALRM,wakeup);
    alarm(4);
    pause();
    printf("Morning so soon\n");
    return 0;
}

void wakeup(int signum){
    # ifdef SHHH
        printf("Alarm received from kernel\n");
    # endif
}