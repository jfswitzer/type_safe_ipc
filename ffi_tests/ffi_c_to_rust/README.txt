An incorrect implementation of an FFI interface, in which the Rust and C programs hold non-equivalent definitions for the type of the shared object. Specifically, the C program believes that the SyncKeysC.xcs field is 8 bits, while the Rust program believes that the SyncKeysC.xcs field is 64 bits; the Rust program also believes that SyncKeysC has an extra, 64 bit field called oops.

Expected output (value that Rust sees for the xcs & oops fields changes between runs):

On C side, sending the values: sync_key 8 xcs 5 
On Rust side, receiving the values: sync_key 0 xcs 362539770019100159 oops 8589934603
