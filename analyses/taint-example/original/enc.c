#include <emscripten.h>
#include <string.h>
#include <stdlib.h>

EMSCRIPTEN_KEEPALIVE
char * enc(char * input) {
    char * output = malloc(strlen(input) + 1);
    for (int i=0; input[i] != 0; i++)
        output[i] = 0x42 ^ (0x02 + input[i]);
    return output;
}
