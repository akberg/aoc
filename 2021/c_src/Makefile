
CFLAGS += -Wall -I.
DATE =$(shell date +%-d)

OPT ?= 0

ifeq ($(OPT), 0)
	CFLAGS += -g
endif

all: run

main-serial: $(DATE)-serial.c
main-pthread: $(DATE)-pthread.c
main-omp: $(DATE)-omp.c

%.serial: %-serial.c tlhash.c
	gcc -O$(OPT) $(CFLAGS) -o main-serial $^

%.pthread: %-pthread.c
	gcc -o$(OPT) $(CFLAGS) -o main-pthread $^ -lpthread

%.omp: %-omp.c
	gcc -o$(OPT) $(CFLAGS) -o main-omp $^ -fopenmp

run: main-serial
	-time ./main-serial

run-pthread: main-pthread
	-time ./main-pthread

run-omp: main-omp
	-time ./main-omp