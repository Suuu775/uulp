#include<stdio.h>
#include<unistd.h>
#include<stdlib.h>
#include<sys/types.h>
#include<sys/wait.h>

#define DELAY 10

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

// parent wait for chlid then print a message
// high_8 is exit value,bit_7 is core dump flag,low_7 is signal number
void parent_code(int chlidpid){
    int wait_rv;
    int chlid_status;
    int high_8,bit_7,low_7;

    wait_rv = wait(&chlid_status);
    printf("done waiting for %d. Wait returned %d\n",chlidpid,wait_rv);

    high_8 = chlid_status >> 8;
    low_7 = chlid_status & 0x7F;
    bit_7 = chlid_status & 0x80;

    printf("status: exit=%d,sig=%d,core=%d\n",high_8,low_7,bit_7);
}