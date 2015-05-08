build normal rust back-end code as rlib.
```
rustlib -> cargo build -> rlib
```
build a rust-wrapper to suppress mangling and provide a c-interface.
```
rust_wrapper -> cargo build --release -> staticlib {Note: debug builds do not build static libraries}
```

Write the C-interface to call into rust-wrapper and compile the whole thing into a shared-object.
```
gcc -c -std=c99 -Wall -Werror -fPIC api.c
gcc -shared -o libcwrapper.so api.o -L./ -lrust_wrapper
```

code a C executable to test the whole thing.
```
gcc -std=c99 -Wall -Werror -O2 main.c -L./ -lcwrapper -lm -ldl
```

Use Swig to port it to other languages - Eg., Python.
```
swig -python interface.i
gcc -std=c99 -c -fPIC -Wall -Werror -O2 api.c interface_wrap.c -I/usr/include/python2.7
gcc -shared -o _port_sample.so api.o interface_wrap.o -L./ -lrust_wrapper
```

Use the shared object from python.
```
spandan@spandan-Lenovo-Y50-70:~/virtualization/coding/my/rust-tests/ffi/test0/c-code/python-swig$ python
Python 2.7.6 (default, Mar 22 2014, 22:59:56) 
[GCC 4.8.2] on linux2
Type "help", "copyright", "credits" or "license" for more information.
>>> import port_sample
>>> handle = port_sample.new_test_struct(36)
>>> port_sample.test_struct_decrement(handle, 12)
>>> value = port_sample.test_struct_trait_function(handle)
>>> print value
27
>>> exit()
```
