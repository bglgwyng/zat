#include <stdio.h>
#include <stdlib.h>

typedef unsigned int uint;

struct Config {
    char *name;
    int value;
};

enum Color {
    RED,
    GREEN,
    BLUE
};

struct Node;

void greet(const char *name) {
    printf("Hello, %s!\n", name);
}

int add(int a, int b) {
    return a + b;
}

char *get_name(struct Config *config);
