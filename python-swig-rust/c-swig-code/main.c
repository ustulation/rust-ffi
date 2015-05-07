#include "api.h"
#include <stdio.h>

int main() {
  printf("%d\n", free_function(36));

  void* test_struct = new_test_struct(36);
  test_struct_decrement(test_struct, 12);
  printf("%d\n", test_struct_trait_function(test_struct));
  return 0;
}
