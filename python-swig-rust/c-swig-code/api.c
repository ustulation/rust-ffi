#include "api.h"

extern int32_t free_function_wrapper(int32_t value);

extern void* new_test_struct_wrapper(int32_t value);
extern void test_struct_decrement_wrapper(void* ptr, int32_t delta);
extern int32_t test_struct_trait_function_wrapper(void* ptr);

int32_t free_function(int32_t value) {
  return free_function_wrapper(value);
}

void* new_test_struct(int32_t value) {
  return new_test_struct_wrapper(value);
}

void test_struct_decrement(void* ptr, int32_t value) {
  test_struct_decrement_wrapper(ptr, value);
}

int32_t test_struct_trait_function(void* ptr) {
  return test_struct_trait_function_wrapper(ptr);
}

