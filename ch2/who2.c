#include<stdio.h>
#include<utmp.h>
#include<time.h>
#include<fcntl.h>
#include<unistd.h>
#include <stdlib.h>

// #define SHOWHOST
void show_info(struct utmp *utbufp);
void show_time(time_t time);

int main()
{
    struct utmp current_record;
    int utmpfd;
    int reclen = sizeof(current_record);

    if ((utmpfd = open(UTMP_FILE,O_RDONLY)) == -1)
    {
        perror(UTMP_FILE);
        exit(1);
    }
    
    while (read(utmpfd,&current_record,reclen)==reclen)
    {
        show_info(&current_record);
    }
    close(utmpfd);
    return 0;
}

void show_info(struct utmp *utbufp){
    if (utbufp->ut_type!=USER_PROCESS)
    {
        return;
    }
    printf("%-8.8s",utbufp->ut_user);
    printf(" ");
    printf("%-8.8s",utbufp->ut_line);
    printf(" ");
    show_time(utbufp->ut_tv.tv_sec);
    printf(" ");
    #ifdef SHOWHOST
        if (utbufp->ut_host[0]!='\0')
        {
            printf("(%s)",utbufp->ut_host);
        }
        
    #endif
    printf("\n");
}

void show_time(time_t time){
    struct tm *tm_info;
    char buffer[20];
 
    tm_info = localtime(&time);
    strftime(buffer, sizeof(buffer), "%Y-%m-%d %H:%M", tm_info);
 
    printf("%s", buffer);
}