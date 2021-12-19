#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>

#include "tlhash.h"

#define IDSTART '<'
#define IDEND   '>'
#define NOTWICE '-'

void input(tlhash_t *ret)
{

    FILE *fp = fopen("../inputs/day12.txt", "r");

    if (fp == NULL) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }
    fprintf(stderr, "Opened file\n");

    ssize_t res;
    char *line = NULL;
    char *tok_ptr = NULL;
    size_t len = 0;

    while ((res = getline(&line, &len, fp)) != -1) {
        /* Read line to array */
        char *a = strtok(line, "-");
        *a = strcmp(a, "start") ? strcmp(a, "end") ? a[0] : IDEND : IDSTART;
        char *b = strtok(NULL, "-");
        *b = strcmp(b, "start") ? strcmp(b, "end") ? b[0] : IDEND : IDSTART;
        tlhash_insert(ret, a, 1, b);
        tlhash_insert(ret, b, 1, a);
    }
    free(line);
    fclose(fp);
}

int bfs_recursive(tlhash_t *graph, char *path, int path_len) {


}

int bfs_recursive2(tlhash_t *graph, char *path, int path_len) {

}

int part1(const tlhash_t *inputs)
{
    char path[] = {IDSTART};
    bfs_recursive(inputs, path, 1);
}

int part2(const char **inputs)
{
    return 0;
}

void main()
{
    tlhash_t *inputs = malloc(sizeof(tlhash_t));
    tlhash_init(inputs, 100);
    input(&inputs);
    char *val;
    char key = IDSTART;
    tlhash_lookup(inputs, &key, 1, &val);
    printf("start: %c, %c", val[0], val[1]);
    int p1, p2;
    p1 = part1(&inputs);
    printf("Part 1: %d\n", p1);
    p2 = part2(&inputs);
    printf("Part 2: %d\n", p2);
}