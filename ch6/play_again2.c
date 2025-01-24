// set tty into char-by-char mode and no-echo mode,read char, return result

#include <stdio.h>
#include <termios.h>

#define QUESTION "Do you want another translation"

int get_response(char * question){

    int input;
    printf("%s (y/n)?",question);
    while (1)
    {
        switch (input = getchar())
        {
            case 'y':
            case 'Y':return 0;
            case 'n':
            case 'N':
            case EOF:return 1;
        }
    }
    
}

// put file descriptor 0 (i.e. stdin) into chr-by-chr mode
void set_cr_noecho_mode(){
    struct termios ttystate;
    tcgetattr(0,&ttystate);
    ttystate.c_lflag &= ~ICANON;
    ttystate.c_lflag &= ~ECHO;
    ttystate.c_cc[VMIN] = 1;
    tcsetattr(0,TCSANOW,&ttystate);
}

// how == 0 => save current mode, how == 1 => restore mode
int tty_mode(int how){
    static struct termios original_mode;
    if (how == 0)
    {
        tcgetattr(0,&original_mode);
    }
    else
    {
        return tcsetattr(0,TCSANOW,&original_mode);
    }
}

int main(int argc, char const *argv[])
{
    int response;
    tty_mode(0);
    set_cr_noecho_mode();
    response = get_response(QUESTION);
    tty_mode(1);
    return response;  
}