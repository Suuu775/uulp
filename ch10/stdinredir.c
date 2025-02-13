#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <unistd.h>

int main(int argc, char const *argv[])
{
    char line[100];

    // raed and print three lines
    fgets(line, 100, stdin);
    printf("%s", line);
    fgets(line, 100, stdin);
    printf("%s", line);
    fgets(line, 100, stdin);
    printf("%s", line);

    // redirect input to file fd
    close(0);
    int fd = open("/etc/passwd", O_RDONLY);
    if (fd != 0)
    {
        fprintf(stderr, "Could not open data as fd 0\n");
        exit(1);
    }

    // raed and print three lines
    fgets(line, 100, stdin);
    printf("%s", line);
    fgets(line, 100, stdin);
    printf("%s", line);
    fgets(line, 100, stdin);
    printf("%s", line);

    return 0;
}
