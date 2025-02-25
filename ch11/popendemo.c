#include <wait.h>
#include <stdio.h>
#include <fcntl.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>

int main(int argc, char const *argv[])
{
    char buf[100];
    int i=0;

    FILE *fp = popen("who|sort","r");
    
    while (fgets(buf,100,fp)!=NULL)
    {
        printf("%3d %s",i++,buf);
    }
    
    pclose(fp);
    return 0;
}
