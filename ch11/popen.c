#include <wait.h>
#include <stdio.h>
#include <fcntl.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>

#define READ 0
#define WRITE 1

FILE *popen(const char *command, const char *mode)
{
    int pfp[2], pid;
    FILE *fp;
    int parent_end, child_end;

    if (*mode == 'r')
    {
        parent_end = READ;
        child_end = WRITE;
    }
    else if (*mode == 'w')
    {
        parent_end = WRITE;
        child_end = READ;
    }
    else
    {
        return NULL;
    }

    if (pipe(pfp) == -1) // get a pipe
    {
        return NULL;
    }

    if ((pid = fork()) == -1) // and a process
    {
        close(pfp[0]); // or dispose of pipe
        close(pfp[1]);
        return NULL;
    }

    /* parent code here */
    /* need to close one end and fdopen other one */
    if (pid > 0)
    {
        if (close(pfp[child_end]) == -1)
        {
            return NULL;
        }
        return fdopen(pfp[parent_end], mode);
    }

    /* child code here */
    /*   need to redirect stdin or stdout then exec the cmd	 */

    if (close(pfp[parent_end]) == -1) /* close the other end	*/
        exit(1);                      /* do NOT return	*/

    if (dup2(pfp[child_end], child_end) == -1)
        exit(1);

    if (close(pfp[child_end]) == -1) /* done with this one	*/
        exit(1);
    /* all set to run cmd	*/
    execl("/bin/sh", "sh", "-c", command, NULL);
    exit(1);
}

int main(int argc, char const *argv[])
{

    return 0;
}
