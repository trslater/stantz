CC=gcc
CFLAGS=-Iinc -Wall -F /Library/Frameworks
LDFLAGS=-Iinc -framework SDL2 -F /Library/Frameworks -I /Library/Frameworks/SDL2.framework/Headers

bin/test: obj/test.o obj/rendering.o
	mkdir -p bin
	$(CC) -o $@ $^ $(LDFLAGS)

obj/%.o: src/%.c inc/%.h
	mkdir -p obj
	$(CC) -c -o $@ $< $(CFLAGS)

.PHONY: clean
clean:
	rm -f bin/*
	rm -f obj/*
