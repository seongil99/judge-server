#include <stdio.h>

int main(int argc, char **argv)
{
    char str[100];
    int i = 0, j;
    scanf("%s", str);
    while(str[i] != '\0')
        i++;
    for(j = i - 1; j >= 0; j--)
    {
        printf("%c", str[j]);
    }    
    printf("\n");
    return 0;
}
