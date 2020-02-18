An incorrect implementation of an FFI interface, in which the Rust and C programs hold non-equivalent definitions for the type of the shared object. Specifically, the C program believes that the SyncKeysC.xcs field is 32 bits, while the Rust program believes that the SyncKeysC.xcs field is 8 bits.

Expected output (value that C sees for xcs changes between runs):

On Rust side, sending the values: sync_key 6 xcs 7
On C side, received the values: sync_key 6 xcs 7ffd 
