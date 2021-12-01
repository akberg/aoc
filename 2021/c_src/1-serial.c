#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef struct {
    int len;
    int *vals;
} list_t;

void input(list_t *ret)
{
    ret->len = 0;
    ret->vals = (int *)malloc(sizeof(int) * 2000);
    int allocated_size = 2000;

    FILE *fp = fopen("../inputs/day1.txt", "r");

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
        ret->vals[ret->len++] = atoi(line);

        if (ret->len >= allocated_size) {
            /* Dynamic allocation of array size */
            fprintf(stderr, "%d / %d full. Allocating more memory to input array\n", 
                    ret->len, allocated_size);
            allocated_size *= 2;
            ret->vals = realloc(ret->vals, sizeof(int) * allocated_size);
            
        }
    }
    free(line);
    fclose(fp);
}

int count_increasing_sums(const list_t *vals)
{
    int count = 0;
    for (int i = 1; i < vals->len; i++)
        count += vals->vals[i] > vals->vals[i-1];
    return count;
}

void make_window(list_t *list)
{
    for (int i = 2; i < list->len; i++) {
        int s = list->vals[i] + list->vals[i-1] + list->vals[i-2];
        list->vals[i-2] = s;
    }
    list->len -= 2;
}

int part1(list_t *depths)
{
    return count_increasing_sums(depths);
}

int part2(list_t *depths)
{
    make_window(depths);
    return count_increasing_sums(depths);
}

void main()
{
    list_t depths;
    input(&depths);
    int p1;
    for (int i = 0; i < 1000; i++) {
        p1 = part1(&depths);

    }
    printf("Part 1: %d\n", p1);
    // int p2 = part2(&depths);
    // printf("Part 2: %d\n", p2);
}