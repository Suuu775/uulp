#include <ctype.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

struct arg_set {
  char *fname;
  int count;
};

int main(int ac, char *av[]) {
  pthread_t t1, t2;
  struct arg_set args1, args2;
  void *count_words(void *);

  if (ac != 3) {
    printf("usage: %s file1 file2\n", av[0]);
    exit(1);
  }
  args1.fname = av[1];
  args1.count = 0;
  pthread_create(&t1, NULL, count_words, (void *)&args1);

  args2.fname = av[2];
  args2.count = 0;
  pthread_create(&t2, NULL, count_words, (void *)&args2);

  pthread_join(t1, NULL);
  pthread_join(t2, NULL);
  printf("%5d: %s\n", args1.count, av[1]);
  printf("%5d: %s\n", args2.count, av[2]);
  printf("%5d: total words\n", args1.count + args2.count);
}
void *count_words(void *a) {
  struct arg_set *args = a;
  FILE *fp;
  int c, prevc = '\0';

  if ((fp = fopen(args->fname, "r")) != NULL) {
    while ((c = getc(fp)) != EOF) {
      if (!isalnum(c) && isalnum(prevc))
        args->count++;
      prevc = c;
    }
    fclose(fp);
  } else
    perror(args->fname);
  return NULL;
}
