An incorrect implementation of an FFI interface, in which the Rust and C programs hold non-equivalent definitions for the type of the shared object. Specifically, the ordering of the sync_key and xcs fields are swapped.

Expected output:

On Rust side, sending the values: SyncKeysC { sync_key: 6, xcs: 7 }
On C side, received the values: sync_key 7, xcs 6
