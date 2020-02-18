#include <stdint.h>
#include <stdio.h>

extern char ffi_test();

/*typedef struct SyncKeysC {
  char sync_key;
  char xcs;
  
  };*/

int main() {
  //struct SyncKeysC syncKey;
  char syncKey = ffi_test();
  printf("Received the values: %d", syncKey);
  return 0;
}
