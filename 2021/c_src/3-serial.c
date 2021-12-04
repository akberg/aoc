#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>

static int width = 0; // Set during input

typedef unsigned long list_type;
typedef struct {
    size_t len;
    list_type *vals;
} list_t;

void input(list_t *ret)
{
    ret->len = 0;
    ret->vals = (int *)malloc(sizeof(list_type) * 2000);
    int allocated_size = 2000;

    FILE *fp = fopen("../inputs/day3.txt", "r");

    if (fp == NULL) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }
    fprintf(stderr, "Opened file\n");

    ssize_t res;
    char *line = NULL;
    size_t len = 0;
    while ((res = getline(&line, &len, fp)) != -1) {
        /* Read line to array */
        ret->vals[ret->len++] = strtoul(line, NULL, 2);
        if (!width) {
            width = strlen(line) - 1; // Ignore newline;
        }

        if (ret->len >= allocated_size) {
            /* Dynamic allocation of array size */
            fprintf(stderr, "%d / %d full. Allocating more memory to input array\n", 
                    ret->len, allocated_size);
            allocated_size *= 2;
            ret->vals = realloc(ret->vals, sizeof(list_type) * allocated_size);
            
        }
    }
    free(line);
    fclose(fp);
}

uint64_t part1(const list_t *inputs)
{
    int mid = inputs->len / 2;
    uint64_t gamma[width];

    
    for (int r = 0; r < width; r++) {
        gamma[r] = 0;
        for (int i = 0; i < inputs->len; i++) {
            gamma[r] += inputs->vals[i] & (1 << (width-r-1)) ? 1 : 0;
        }
    }
    uint64_t g = 0;
    for (int r = 0; r < width; r++) {
        if (gamma[r] > mid) {
            g += 1 << (width-r-1);
        }
    }
    return g * (g ^ ((1 << width)-1));
}

uint64_t part2(list_t *inputs)
{
    
}

void main()
{
    list_t inputs;
    input(&inputs);
    uint64_t p1;
    p1 = part1(&inputs);
    printf("Part 1: %d\n", p1);
    int p2 = part2(&inputs);
    printf("Part 2: %d\n", p2);
}