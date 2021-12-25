.PHONY: all clean

all: main.x

test.c: transpile.py mine.txt
	python3 transpile.py <mine.txt >$@

%.o: %.c
	gcc --std=c99 -march=native -Ofast -c -o $@ $<

main.x: main.o test.o
	gcc -o $@ $^

clean:
	$(RM) -f *.o *.x test.c
