#include<stdio.h>
#include<dirent.h>
#include<string.h>
#include <stdlib.h>
#include <unistd.h>
#include<sys/stat.h>
#include<sys/types.h>

ino_t	get_inode(char *);
void    inum_to_name(ino_t , char *, int );
void    printpathto(ino_t);

int main(){
    printpathto(get_inode( "." ));
    putchar('\n');
    return 0;
}

void printpathto( ino_t this_inode ){
    /*
    * prints path leading down to an object with this inode
    * kindof recursive
    */
    ino_t my_inode;
    char its_name[BUFSIZ];
    if (get_inode("..")!=this_inode)
    {
        chdir("..");
        inum_to_name(this_inode,its_name,BUFSIZ);
        my_inode = get_inode(".");
        printpathto(my_inode);
        printf("/%s",its_name);
    }
    
}

void inum_to_name(ino_t inode_to_find,char * namebuf ,int buflen){
    /*
    looks through current dirctory for a file with this code
    number and copies its name into namebuf
    */
   DIR *dir_ptr;
   struct dirent * direntp;
   dir_ptr = opendir(".");
   if (dir_ptr == NULL)
   {
        perror(".");
        exit(1);
   }
   while ((direntp = readdir(dir_ptr)) != NULL)
   {
        if (direntp->d_ino == inode_to_find)
        {
            strncpy(namebuf,direntp->d_name,buflen);
            namebuf[buflen-1] = '\0';
            closedir(dir_ptr);
            return;
        }
        
   }
   fprintf(stderr,"error looking for %d\n",inode_to_find);
   exit(1);
}

ino_t get_inode(char * fname){
    // return the inode of fname
    struct stat info;
    if (stat(fname,&info) == -1)
    {
        fprintf(stderr,"Can't stat");
        perror(fname);
        exit(1);
    }
    return info.st_ino;
}