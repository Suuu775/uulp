#include <stdio.h>
#include <termios.h>
#include <fcntl.h>
#include <string.h>
#include <unistd.h>
#include <ctype.h>

#define ASK "Do you want another translation"
#define TRIES 3
#define SLEEPTIME 2
#define BEEP putchar('\a')

// how == 0 => save current mode, how == 1 => restore mode
int tty_mode(int how)
{

    static struct termios original_mode;
    static int original_flags;
    if (how == 0)
    {
        tcgetattr(0, &original_mode);
        original_flags = fcntl(0, F_GETFL);
    }
    else
    {
        tcsetattr(0, TCSANOW, &original_mode);
        fcntl(0, F_SETFL, original_flags);
    }
}

// put stdio into no delay mode
void set_nodelay_mode()
{
    int termflags;
    termflags = fcntl(0, F_GETFL);
    termflags |= O_NDELAY;
    fcntl(0, F_SETFL, termflags);
}

// put file descriptor 0 (i.e. stdin) into chr-by-chr mode
void set_cr_noecho_mode()
{
    struct termios ttystate;
    tcgetattr(0, &ttystate);
    ttystate.c_lflag &= ~ICANON;
    ttystate.c_lflag &= ~ECHO;
    ttystate.c_cc[VMIN] = 1;
    tcsetattr(0, TCSANOW, &ttystate);
}

int get_ok_char()
{
    int c;
    while ((c = getchar()) != EOF && strchr("yYnN", c) == NULL)
    {
        ;
    }
    return c;
}

// ask a question and wait for a y/n answer or maxtries
int get_response(char *question, int maxtries)
{
    int input;

    printf("%s (y/n)?", question);
    fflush(stdout);
    while (1)
    {
        sleep(SLEEPTIME);
        input = tolower(get_ok_char());
        if (input == 'y')
            return 0;
        if (input == 'n')
            return 1;
        if (maxtries-- == 0)
            return 2;
        BEEP;
    }
}

int main()
{
    int response;

    tty_mode(0);
    set_cr_noecho_mode();
    set_nodelay_mode();
    response = get_response(ASK, TRIES);
    tty_mode(1);
    return response;
}