#include <stdio.h>
#include<string.h>
#include<signal.h>
#include <unistd.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>

#define MAXARGS 20 // cmdline args
#define ARGLEN 100 // token length

void execute(char *[]);
char *makestring(char *);

int main(int argc, char const *argv[])
{
    signal(SIGINT,SIG_IGN);
    signal(SIGQUIT,SIG_IGN);
    char *arglist[MAXARGS + 1];
    int numargs = 0;
    char argbuf[ARGLEN];

    while (numargs < MAXARGS)
    {
        printf("Arg[%d]?", numargs);
        if (fgets(argbuf, ARGLEN, stdin) && *argbuf != '\n')
        {
            if (numargs==0&&!strncmp(argbuf,"exit\n",4))
            {
                return 0;
            }
            
            arglist[numargs++] = makestring(argbuf);
        }
        else
        {
            if (numargs > 0)
            {
                arglist[numargs] = NULL;
                execute(arglist);
                numargs = 0;
            }
        }
    }

    return 0;
}

// use execvp to do it
void execute(char *arglist[])
{
    int pid, exitstatus;
    pid = fork();

    switch (pid)
    {
    case -1:
        perror("fork failed");
        exit(1);
    case 0:
        execvp(arglist[0], arglist);
        perror("execvp failed");
        exit(1);
    default:
        while (wait(&exitstatus) != pid)
            ;
        printf("child exited with status %d,%d\n",
               exitstatus >> 8, exitstatus & 0377);
    }
}

// trim off newline and create storage for the string
char *makestring(char *buf)
{
    char *cp;
    buf[strlen(buf) - 1] = '\0';
    cp = malloc(strlen(buf) + 1);
    if (cp == NULL)
    {
        fprintf(stderr, "no memory");
        exit(1);
    }
    strcpy(cp, buf);
    return cp;
}