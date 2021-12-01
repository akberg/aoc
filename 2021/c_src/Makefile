
CFLAGS += -Wall

OPT ?= 0

ifeq ($(OPT), 0)
	CFLAGS += -g
endif

%.serial: %-serial.c
	gcc -O$(OPT) $(CFLAGS) -o main-serial $^

%.pthread: %-pthread.c
	gcc -o$(OPT) $(CFLAGS) -o main-pthread $^ -lpthread

run: main-serial
	-time ./main-serial

run-pthread: main-pthread
	-time ./main-pthread