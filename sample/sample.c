#include <stdio.h>
#include "foo.h"

int main() {
    foo();
    printf("Hello, world!");
    getchar(); // Hold the terminal open.
}