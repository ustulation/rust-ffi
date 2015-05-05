#include "api.h"

extern int32_t free_function_wrapper(int32_t value);

int32_t free_function(int32_t value) {
  return free_function_wrapper(value);
}
