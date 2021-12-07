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

    FILE *fp = fopen("../inputs/day7.txt", "r");

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

int list_max(const list_t *ls) {
    int mx = INT32_MIN;
    for (int i = 0; i < ls->len; i++) {
        mx = mx > ls->vals[i] ? mx : ls->vals[i];
    }
    return mx;
}

/**Calculate median and sum all numbers' difference to it */
int part1(const list_t *inputs)
{
    // Count sort to calculate median
    int sz = list_max(inputs);
    int count[sz];
    memset(&count, 0, sizeof(int)*sz);

    for (int i = 0; i < inputs->len; i++) {
        count[inputs->vals[i]] += 1;
    }
    int c = 0, median = 0;
    while (c < inputs->len / 2) {
        c += count[median++];
    }

    
}

int part2(list_t *inputs)
{
    return evolve(inputs, 80);
}

void main()
{
    list_t inputs;
    input(&inputs);
    int p1, p2;
    p1 = part1(&inputs);
    printf("Part 1: %d\n", p1);
    p2 = part2(&inputs);
    printf("Part 2: %d\n", p2);
}