#include <wait.h>
#include <stdio.h>
#include <fcntl.h>
#include<string.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>

int main(int argc, char const *argv[])
{
    int len,i,apipe[2];
    char buf[BUFSIZ];

    // get a pipe
    if (pipe(apipe) == -1)
    {
        perror("could not make pipe");
        exit(1);
    }

    printf("Got a pipe! It is file descriptions:[%d,%d]\n",
        apipe[0],apipe[1]);
    
    // read from stdin,write into pipe,read from pipe print it
    while (fgets(buf,BUFSIZ,stdin))
    {
        len = strlen(buf);
        if (write(apipe[1],buf,len)!=len)
        {
            perror("writing to pipe");
            break;
        }
        
        for (int i = 0; i < len; i++)
        {
            buf[i]='X';
        }
        
        len = read(apipe[0],buf,BUFSIZ);

        if (len == -1)
        {
            perror("reading to pipe");
            break;
        }

        if (write(1,buf,len)!=len)
        {
            perror("writing to stdout");
            break;
        }
    }
    return 0;
}
