ASM = gcc
SRC = main.s
OUT = main

all:
	$(ASM) -nostdlib -no-pie $(SRC) -o $(OUT)

run: all
	./$(OUT)

debug: all
	gdb ./$(OUT)

clean:
	rm -f $(OUT)
