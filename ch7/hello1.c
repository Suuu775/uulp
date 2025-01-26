#include<stdio.h>
#include<curses.h>

int main(int argc, char const *argv[])
{
    initscr();

    clear();
    move(10,20);
    addstr("Hello World");
    move(LINES-1,0);
    refresh();
    getch();
    endwin();
    return 0;
}
