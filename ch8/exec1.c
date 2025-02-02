#include<stdio.h>
#include<unistd.h>
int main(int argc, char const *argv[])
{
    char *arglist[3];
    arglist[0] = "ls";
    arglist[1] = "-l";
    arglist[2] = 0;
    printf("*** about to exec ls -l\n");
    execvp("ls",arglist);
    printf("*** ls is done.bye\n");
    return 0;
}
