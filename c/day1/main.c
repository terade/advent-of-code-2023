#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INPUT_FILE "input.sf"
// #define INPUT_FILE "test.sf"
#define MAX_LINE_LENGTH 200
const char* digits_written[] = {"one", "two",   "three", "four", "five",
                                "six", "seven", "eight", "nine"};

char* read_line(FILE* fptr) {
    char* line = malloc(MAX_LINE_LENGTH * sizeof(char));
    line = fgets(line, MAX_LINE_LENGTH * sizeof(char), fptr);
    return line;
}

int get_num_line(char* line) {
    int first = 0;
    int last = 0;
    unsigned len = strlen(line);

    for (int i = 0; i < len; i++) {
        if (line[i] <= '9' && line[i] >= '0') {
            first = line[i] - '0';
            break;
        }
    }
    for (int i = len - 1; i >= 0; i--) {
        if (line[i] <= '9' && line[i] >= '0') {
            last = line[i] - '0';
            break;
        }
    }

    return (first * 10) + last;
}

int part1() {
    FILE* input_file_ptr = fopen(INPUT_FILE, "r");
    unsigned sum = 0;

    if (input_file_ptr == NULL) {
        printf("could not read from file\n");
        return -1;
    }

    char* line = NULL;

    while (line = read_line(input_file_ptr)) {
        sum += get_num_line(line);
        free(line);
    }

    fclose(input_file_ptr);
    return sum;
}

int part2() {
    FILE* input_file_ptr = fopen(INPUT_FILE, "r");
    unsigned sum = 0;

    if (input_file_ptr == NULL) {
        printf("could not read from file\n");
        return -1;
    }

    char* line = NULL;

    while (line = read_line(input_file_ptr)) {
        unsigned len = strlen(line);
        int first = 0;
        int last = 0;

        for (int i = 0; i < len; i++) {
            for (int j = 0;
                 j < sizeof(digits_written) / sizeof(digits_written[0]); j++) {
                if (0 == strncmp(line + i, digits_written[j],
                                 strlen(digits_written[j]))) {
                    line[i] = j + 1 + '0';
                }
            }
        }

        sum += get_num_line(line);
        free(line);
    }

    fclose(input_file_ptr);
    return sum;
}

int main(int argc, char* argv) {
    printf("%d\n", part1());
    printf("%d\n", part2());
    return 0;
}
