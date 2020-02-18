A correct implementation of an FFI interface, in which the Rust and C programs hold equivalent definitions for the type of the shared object.

Expected output:

On Rust side, sending the values: SyncKeysC { sync_key: 6, xcs: 7 }
On C side, received the values: sync_key 6, xcs 7 
