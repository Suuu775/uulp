/* play_again0.c
 *	purpose: ask if user wants another transaction
 *	 method: ask a question, wait for yes/no answer
 *	returns: 0=>yes, 1=>no
 *	 better: eliminate need to press return 
 */

#include<stdio.h>
#include<termios.h>

#define QUESTION "Do you want another translation"

int get_response(char *);

int main(int argc, char const *argv[])
{
    int response;
    response = get_response(QUESTION);
    return response;
    return 0;
}

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