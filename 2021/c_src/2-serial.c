#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>

typedef enum { UP, DOWN, FORWARD } direction_e;

typedef struct {
    direction_e dir;
    int len;
} command_t;

typedef command_t list_type;
typedef struct {
    size_t len;
    list_type *vals;
} list_t;

void input(list_t *ret)
{
    ret->len = 0;
    ret->vals = (int *)malloc(sizeof(list_type) * 2000);
    int allocated_size = 2000;

    FILE *fp = fopen("../inputs/day2.txt", "r");

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
        command_t cmd;
        char *saveptr;
        char *tok = strtok_r(line, " ", &saveptr);
        //fprintf(stderr, "Direction: %s", tok);
        if (!strcmp(tok, "forward")) {
            cmd.dir = FORWARD;
        }
        else if (!strtok(tok, "up")) {
            cmd.dir = UP;
        }
        else if (!strtok(tok, "down")) {
            cmd.dir = DOWN;
        }
        //fprintf(stderr, ", ");

        tok = strtok_r(NULL, " ", &saveptr);
        cmd.len = atoi(tok);
        //fprintf(stderr, "Length: %d = %s", cmd.len, tok);

        ret->vals[ret->len++] = cmd;

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

uint64_t part1(list_t *cmds)
{
    uint64_t x = 0; // Horizontal position
    uint64_t y = 0; // Vertical position/depth

    for (int i = 0; i < cmds->len; i++) {
        int len = cmds->vals[i].len;
        switch (cmds->vals[i].dir) {
            case FORWARD: x += len; break;
            case UP: y -= len; break;
            case DOWN: y += len; break;
        }
    }
    return x * y;
}

uint64_t part2(list_t *cmds)
{
    uint64_t x = 0; // Horizontal position
    uint64_t y = 0; // Vertical position/depth
    uint64_t a = 0; // Aim

    for (int i = 0; i < cmds->len; i++) {
        int len = cmds->vals[i].len;
        switch (cmds->vals[i].dir) {
            case FORWARD: x += len; y += a * len; break;
            case UP: a -= len; break;
            case DOWN: a += len; break;
        }
    }
    return x * y;
}

void main()
{
    list_t inputs;
    input(&inputs);
    int p1;
    for (int i = 0; i < 1000; i++) {
        p1 = part1(&inputs);

    }
    printf("Part 1: %d\n", p1);
    int p2 = part2(&inputs);
    printf("Part 2: %d\n", p2);
}