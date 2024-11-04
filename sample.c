#include <stdio.h>
#include <string.h>

/* This program demonstrates the use of some string functions in C. */
/*
A multi-line comment!
*/

int main() {
    char str[] = "Hello, World!";
    int length = strlen(str);

    printf("Original string: %s\n", str);
    printf("String length: %d\n", length);

    printf("Reversed string: ");
    for (int i = length - 1; i >= 0; i--) {
        putchar(str[i]);
    }
    printf("\n");

    printf("ASCII values:\n");
    for (int i = 0; i < length; i++) {
        printf("'%c' -> %d\n", str[i], str[i]);
    }

    return 0;
}