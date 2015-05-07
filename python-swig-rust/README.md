rustlib -> cargo build -> rlib
rust_wrapper -> cargo build --release -> staticlib {Note: debug builds do not build static libraries}


gcc -c -std=c99 -Wall -Werror -fPIC api.c
gcc -shared -o libcwrapper.so api.o -L./ -lrust_wrapper
gcc -std=c99 -Wall -Werror -O2 main.c -L./ -lcwrapper -lm -ldl

swig -python interface.i
gcc -std=c99 -c -fPIC -Wall -Werror -O2 api.c interface_wrap.c -I/usr/include/python2.7
gcc -shared -o _port_sample.so api.o interface_wrap.o -L./ -lrust_wrapper

spandan@spandan-Lenovo-Y50-70:~/virtualization/coding/my/rust-tests/ffi/test0/c-code/python-swig$ python
Python 2.7.6 (default, Mar 22 2014, 22:59:56) 
[GCC 4.8.2] on linux2
Type "help", "copyright", "credits" or "license" for more information.
>>> import port_sample
>>> var = port_sample.new_test_struct(36)
>>> port_sample.test_struct_decrement(var, 12)
>>> value = port_sample.test_struct_trait_function(var)
>>> print value
27
>>> exit()
