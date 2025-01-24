#include<stdio.h>
#include<signal.h>
#include<unistd.h>

int main(int argc, char const *argv[])
{
    void f(int);
    int i;
    signal(SIGINT,f);
    for ( i = 0; i < 5; i++)
    {
        printf("hello\n");
        sleep(1);
    }
    
    return 0;
}

void f(int signum){
    printf("OOCH! \n");
}