#include <stdio.h>
#include <stddef.h>
#include <string.h>
#include <ctype.h>

void caesar(char *text, int shift) {
    static const int norm = 26;

    shift = ((shift % norm) + norm) % norm;

    for (unsigned char *p = (unsigned char *)text; *p != '\0'; p++) {
        if (*p >= 'A' && *p <= 'Z') {
            *p = 'A' + (*p - 'A' + shift) % norm;
        } else if (*p >= 'a' && *p <= 'z') {
            *p = 'a' + (*p - 'a' + shift) % norm;
        }
    }
}

void xor_cipher(unsigned char *data, size_t data_len, const unsigned char *key, size_t key_len) {
    if (key_len == 0) return;

    for (size_t i = 0; i < data_len; i++) {
        data[i] ^= key[i % key_len];
    }
}

int main(void) {
    char text[] = "Hello, World!";

    // H -> K; e -> h; l -> o; l -> o; o -> r; 
    // W -> Z; o -> r; r -> u; l -> o; d -> g;
    // Khoor, Zruog!
    caesar(text, 3);
    printf("Caesar encryption result = \"%s\"\n", text);

    // K -> H; h -> e; o -> l; o -> l; r -> o; 
    // Z -> W; r -> o; u -> r; o -> l; g -> d;
    // Hello, world!
    caesar(text, -3);
    printf("Caesar decryption result = \"%s\"\n", text);

    static const unsigned char key[] = "abc";
    unsigned char data[] = "Hello, World!";

    size_t key_len = strlen((const char *)key);
    size_t data_len = strlen((const char *)data);

    xor_cipher(data, data_len, key, key_len);
    for (size_t i = 0; i < data_len; i++) {
        unsigned char byte = data[i];
        if (isprint(byte)) printf("%c", byte);
    }
    printf("\n");

    xor_cipher(data, data_len, key, key_len);
    for (size_t i = 0; i < data_len; i++) {
        unsigned char byte = data[i];
        if (isprint(byte)) printf("%c", byte);
    }
    printf("\n");

    return 0;
}
