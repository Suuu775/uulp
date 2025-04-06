#include<stdio.h>
#include<unistd.h>

#define NUM 5

void print_msg(char *);

int main(int argc, char const *argv[])
{
  print_msg("hello ");
  print_msg("world\n");
  return 0;
}

void print_msg(char *m){
  for (size_t i = 0; i < NUM; i++)
  {
    printf("%s",m);
    fflush(stdout);
    sleep(1);
  }
  
}