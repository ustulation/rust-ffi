#include <cstdio>
#include <cstdint>

extern "C" int32_t rust_function(int32_t value);

int main() {
        int32_t value = 99;
        std::printf("The value after calling rust is: %d\n", rust_function(value));
        return 0;
}
