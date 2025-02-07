#include<stdio.h>

extern char	**environ;    /* points to the array of strings */

int main()
{
	int	i;

	for( i = 0 ; environ[i] ; i++ )
		printf("%s\n", environ[i] );
}