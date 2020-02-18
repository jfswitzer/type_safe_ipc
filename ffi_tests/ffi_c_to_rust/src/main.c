#include <stdint.h>
#include <stdio.h>

struct SyncKeysC;
extern void ffi_test(SyncKeysC);

struct SyncKeysC {
  int8_t sync_key;
  int8_t xcs;
};

int main() {
  int secrets1[] = {11.0, 2.0, 3.4, 7.0, 50.0};  
  struct SyncKeysC syncKey;
  syncKey.sync_key = 8;
  syncKey.xcs = 5;
  int secrets2[] = {7.0, 2.0, 3.4, 7.0, 50.0};  
  printf("On C side, sending the values: sync_key %x xcs %x \n", syncKey.sync_key, syncKey.xcs);
  ffi_test(syncKey);
  return 0;
}
