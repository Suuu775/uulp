#include<stdio.h>
#include<sys/stat.h>

int main(int argc, char const *argv[])
{
    struct stat infobuf;
    if (stat("/etc/passwd",&infobuf)==-1)
    {
        perror("/etc/passwd");
    } else
    {
        printf("The size of /etc/passwd is %d\n",infobuf.st_size);
    }
    return 0;
}
