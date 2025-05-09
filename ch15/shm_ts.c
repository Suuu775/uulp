#include <stdio.h>
#include <string.h>
#include <sys/ipc.h>
#include <sys/shm.h>
#include <time.h>
#include <unistd.h>
#include <stdlib.h>

#define TIME_MEM_KEY 99
#define SEG_SIZE ((size_t) 100)
#define oops(m, x)                                                             \
  {                                                                            \
    perror(m);                                                                 \
    exit(x);                                                                   \
  }

int main(){
  int seg_id;
  char *mem_ptr;
  long now;
  int n;

  seg_id = shmget(TIME_MEM_KEY, SEG_SIZE, IPC_CREAT|0777);
  if (seg_id==-1) {
    oops("shmget", 1);
  }

  mem_ptr = shmat(seg_id, NULL, 0);
  if (mem_ptr==(void*) -1) {
    oops("shmat", 2);
  }

  for (n=0; n<60;n++) {
    time(&now);
    strcpy(mem_ptr, ctime(&now));
    sleep(1);
  }
  shmctl(seg_id, IPC_RMID, NULL);
}