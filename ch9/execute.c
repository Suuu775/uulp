#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <signal.h>
#include <sys/wait.h>
#include <sys/types.h>
#include "smsh.h"

/*
 * purpose: run a program passing it arguments
 * returns: status returned via wait, or -1 on error
 *  errors: -1 on fork() or wait() errors

 */
int execute(char *argv[])
{
    int pid;
    int child_info = -1;

    if (argv[0] == NULL) /* nothing succeeds	*/
        return 0;

    if ((pid = fork()) == -1)
        perror("fork");
    else if (pid == 0)
    {
        signal(SIGINT, SIG_DFL);
        signal(SIGQUIT, SIG_DFL);
        execvp(argv[0], argv);
        perror("cannot execute command");
        exit(1);
    }
    else
    {
        if (wait(&child_info) == -1)
            perror("wait");
    }
    return child_info;
}
