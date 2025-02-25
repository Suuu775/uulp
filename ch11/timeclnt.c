#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <netdb.h>
#include <time.h>
#include <string.h>

#define oops(msg)    \
    {                \
        perror(msg); \
        exit(1);     \
    }

int main(int argc, char const *argv[])
{
    struct sockaddr_in servadd;
    struct hostent *hp;
    int sock_id, sock_fd;
    char message[BUFSIZ];
    int messlen;

    // Step 1: Get a socket
    sock_id = (AF_INET, SOCK_STREAM, 0);
    if (sock_id == -1)
    {
        oops("socket");
    }

    //  Step 2: connect to server need to build address (host,port) of server  first

    bzero(&servadd, sizeof(servadd));
    hp = gethostbyname(argv[1]);
    if (hp == NULL)
        oops(argv[1]);
    bcopy(hp->h_addr_list[0], (struct sockaddr *)&servadd.sin_addr, hp->h_length);

    servadd.sin_port = htons(atoi(argv[2]));

    servadd.sin_family = AF_INET;

    if (connect(sock_id, (struct sockaddr *)&servadd, sizeof(servadd)) != 0)
        oops("connect");

    // Step 3: transfer data from server, then hangup

    messlen = read(sock_id, message, BUFSIZ);
    if (messlen == -1)
        oops("read");
    if (write(1, message, messlen) != messlen)
        oops("write");
    close(sock_id);
    return 0;
}
