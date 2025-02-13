#include<stdio.h>

int main(int argc, char const *argv[])
{
    printf("Number of args :%d,Args are:\n",argc);

    for (size_t i = 0; i < argc; i++)
    {
        printf("args[%d] %s\n",i,argv[i]);
    }
    fprintf(stderr,"This message is sent to stderr");
    return 0;
}
