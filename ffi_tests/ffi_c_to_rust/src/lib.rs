#[repr(C)]
#[derive(Debug)]
pub struct SyncKeysC {
    pub sync_key: i8,
    pub xcs: i64,
    pub oops: i64
}

#[no_mangle]
pub extern fn ffi_test(c_struct: SyncKeysC) {
    println!("On Rust side, receiving the values: sync_key {:?} xcs {:?} oops {:?}", c_struct.sync_key, c_struct.xcs, c_struct.oops);
}
