#include<stdio.h>
#include<unistd.h>
#include<curses.h>

int main(int argc, char const *argv[])
{
    int i;
    initscr();

    clear();
    for ( i = 0; i < LINES; i++)
    {
        move(i,2*i);
        if (i%2==1)
        {
            standout();
        }
        addstr("Hello,World");
        if (i%2==1)
        {
            standend();
        }
        sleep(1);
        refresh();
    }
    endwin();
    return 0;
}
