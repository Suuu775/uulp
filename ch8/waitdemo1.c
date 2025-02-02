#include<stdio.h>
#include<unistd.h>
#include<stdlib.h>
#include<sys/types.h>
#include<sys/wait.h>

#define DELAY 2

void parent_code(int);
void child_code(int);

int main(int argc, char const *argv[])
{
    int newpid;

    printf("before: mypid is %d\n",getpid());

    if ((newpid = fork())==-1)
    {
        perror("fork");
    }
    else if (newpid==0)
    {
        child_code(DELAY);
    }
    else
    {
        parent_code(newpid);
    }
    
    return 0;
}

// new process take a nap and then exits
void child_code(int delay){
    printf("child %d here,will sleep %d seconds \n",getpid(),delay);
    sleep(delay);
    printf("child done,about to exit\n");
    exit(17);
}

void parent_code(int chlidpid){
    int wait_rv;
    wait_rv = wait(NULL);
    printf("done waiting for %d. Wait returned %d\n",chlidpid,wait_rv);
}