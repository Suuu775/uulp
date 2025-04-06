#include<stdio.h>
#include<unistd.h>
#include<pthread.h>

#define NUM 5

void* print_msg(void *);

int main(int argc, char const *argv[])
{
  pthread_t t1,t2;
  pthread_create(&t1,NULL,print_msg,(void *)"hello ");
  pthread_create(&t2,NULL,print_msg,(void *)"world\n");
  pthread_join(t1,NULL);
  pthread_join(t2,NULL);
  return 0;
}

void* print_msg(void *m){
  char *cp = (char *) m;
  for (size_t i = 0; i < NUM; i++)
  {
    printf("%s",m);
    fflush(stdout);
    sleep(1);
  }
  return NULL;
}