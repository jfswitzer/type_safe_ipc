#include <stdint.h>
#include <stdio.h>

struct SyncKeysC;
extern struct SyncKeysC ffi_test();

struct SyncKeysC {
  int8_t sync_key;
  int32_t xcs;      
};

int main() {
  struct SyncKeysC syncKey = ffi_test();
  printf("On C side, received the values: sync_key %x xcs %x \n", syncKey.sync_key, syncKey.xcs);
  return 0;
}
