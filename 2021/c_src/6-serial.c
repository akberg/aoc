#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>

static int width = 0; // Set during input

typedef int list_type;
typedef struct {
    size_t len;
    list_type *vals;
} list_t;

void input(list_t *ret)
{
    ret->len = 0;
    int allocated_size = 100;
    ret->vals = (int *)malloc(sizeof(list_type) * allocated_size);

    FILE *fp = fopen("../inputs/day6.txt", "r");

    if (fp == NULL) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }
    fprintf(stderr, "Opened file\n");

    ssize_t res;
    char *line = NULL;
    char *tok_ptr = NULL;
    size_t len = 0;

    res = getline(&line, &len, fp);
    if (res != -1) {
        char *next = strtok_r(line, ",", &tok_ptr);

        while (next != NULL) {
            /* Read line to array */
            printf("%s ", next);
            ret->vals[ret->len++] = atoi(next);

            next = strtok_r(NULL, ",", &tok_ptr);

            if (ret->len >= allocated_size) {
                /* Dynamic allocation of array size */
                fprintf(stderr, "%d / %d full. Allocating more memory to input array\n", 
                        ret->len, allocated_size);
                allocated_size *= 2;
                ret->vals = realloc(ret->vals, sizeof(list_type) * allocated_size);
                
            }
        }
    }
    free(line);
    fclose(fp);
}



uint64_t evolve(const list_t *inputs, int days) {
    uint64_t ocean[9] = {0};
    int offset = 0; // offset to treat array as a circular buffer
    for (int i = 0; i < inputs->len; i++) {
        ocean[inputs->vals[i]] += 1;
    }

    for (int i = 0; i < days; i++) {
        uint64_t new_fish = ocean[offset];
        ocean[(offset+7) % 9] += new_fish;
        ocean[(offset+9) % 9] = new_fish;
        offset = (offset + 1) % 9;
    }
    
    uint64_t colony = 0;
    for (int i = 0; i < 9; i++) {
        colony += ocean[i];
    }
    return colony;
}

uint64_t part1(const list_t *inputs)
{
    return evolve(inputs, 80);
}

uint64_t part2(list_t *inputs)
{
    return evolve(inputs, 80);
}

void main()
{
    list_t inputs;
    input(&inputs);
    uint64_t p1, p2;
    p1 = part1(&inputs);
    printf("Part 1: %d\n", p1);
    p2 = part2(&inputs);
    printf("Part 2: %d\n", p2);
}