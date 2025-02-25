#include <wait.h>
#include <stdio.h>
#include <fcntl.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>

int main(int argc, char const *argv[])
{
    FILE *fp = popen("mail admin backup","w");
    fprintf(fp,"Error with backup!!\n");
    pclose(fp);
    return 0;
}
