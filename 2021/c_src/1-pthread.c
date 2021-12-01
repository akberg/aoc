#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <pthread.h>

/**
 * @brief Parallelized solution using pthreads. Mostly for exam practice,
 * input isn't nearly large enough to gain from parallelizing.
 * 
 */

typedef struct {
    int len;
    int count;
    int thread_idx;
    int *vals;
} list_t;

#define N_THREADS 4
pthread_t threads[N_THREADS];
list_t thread_args[N_THREADS];

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

/**
 * @brief Parallelized counting part of day 1 using pthreads
 * 
 */
void *count_increasing_sums(list_t *vals)
{
    pthread_t self = pthread_self();
    int thread_idx = vals->thread_idx;
    fprintf(stderr, "%u len = %d\n", thread_idx, vals->len);
    for (int i = 0; i < vals->len; i++)
        fprintf(stderr, "%u[%d]: %d\n", thread_idx, i, vals->vals[i]);
    int *count = malloc(sizeof(int));
    for (int i = 1; i < vals->len; i++)
        *count += vals->vals[i] > vals->vals[i-1];
    
    fprintf(stderr, "Thread %u counts %d\n", thread_idx, *count);
    int *tret;
    for (int i = 2; i <= N_THREADS; i *= 2) {
        if (thread_idx % i == 0) {
            /* Join thread with the ones left in previous iteration */
            pthread_join(threads[thread_idx+(i/2)], &tret);
            fprintf(stderr, "Thread %u adds %d from thread %u\n", thread_idx, *tret, thread_idx+(i/2));
            *count += *tret;
        } else { break; }
    }
    fprintf(stderr, "Thread %u returns %d\n", thread_idx, *count);
    pthread_exit(count);
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
    int offset = 0;
    /* Set partition size, assuming round numbers */
    int part = depths->len / N_THREADS;

    for (int i = 0; i < N_THREADS; i++) {
        /* Set offset of partial array */
        thread_args[i].vals = &depths->vals[offset-(i > 0 ? 1 : 0)];
        /* Set length for , catching any remainder in last thread. */
        thread_args[i].len = i < N_THREADS-1 ? part + (i > 0) : depths->len - offset + 1;
        /* Pass thread's index */
        thread_args[i].thread_idx = i;
        offset += part;
        pthread_create(&threads[i], NULL, count_increasing_sums, &thread_args[i]);
    }
    int *count;
    /* Get final result from thread 0 */
    pthread_join(threads[0], &count);
    return *count;
}

int part2(list_t *depths)
{
    make_window(depths);
    return count_increasing_sums(depths);
}

void main()
{
    int arr[] = {199, 200, 208, 210, 200, 207, 240, 269 };
    list_t depths; // = { .len = 8, .vals = arr};
    input(&depths);
    printf("Parsed input.\n");
    int p1;
    for (int i = 0; i < 1000; i++) {
        p1 = part1(&depths);
    }
    printf("Part 1: %d\n", p1);
    // int p2 = part2(&depths);
    // printf("Part 2: %d\n", p2);
}

/*
0 1   1 2 3   3 4 5   6 6 7   7 8 9   9 10 11  11 12 13   13 14 15

    0               2               4                  6
           0                               4
                          0
*/