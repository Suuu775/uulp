// rotate.c:map a->b,b->c...z->a

#include<stdio.h>
#include<ctype.h>

int main(int argc, char const *argv[])
{
    int c;
    while ((c=getchar())!=EOF)
    {
        if (c=='z')
        {
            c='a';
        }
        else if (islower(c))
        {
            c++;
        }
        putchar(c);
    }
    
    return 0;
}
