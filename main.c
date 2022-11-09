#include <stdio.h>
#include <time.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <sys/stat.h>
#include <fcntl.h>

int main() {
    clock_t start = clock();
    printf("Hello world!\n");
    long long i = 1000000000;
    while(i--);

    int a, b;
    scanf("%d %d", &a, &b);
    printf("%d\n", a + b);

    clock_t end = clock();
    printf("Time taken: %lf\n", (double)(end - start) / CLOCKS_PER_SEC);
    return 0;
}