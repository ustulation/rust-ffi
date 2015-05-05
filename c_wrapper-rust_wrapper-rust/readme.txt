gcc -c -std=c99 -Wall -Werror -fPIC api.c
gcc -shared -o libcwrapper.so api.o -L./ -lrust_wrapper
gcc -std=c99 -Wall -Werror -O2 main.c -L./ -lcwrapper
