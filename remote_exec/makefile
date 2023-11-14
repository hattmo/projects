src = src

all: main dummy
	@echo "all done"

dummy: $(src)/dummy.c
	gcc -o dummy $(src)/dummy.c

main: $(src)/main.c $(src)/remote_call.c $(src)/remote_call.h
	gcc -Wall -Wextra -pedantic -Wmissing-prototypes -Wstrict-prototypes -Werror -o main $(src)/main.c $(src)/remote_call.c
