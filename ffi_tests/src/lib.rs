use ffi_support::{self, IntoFfi};
use std::{ptr, os::raw::c_char};
pub struct SyncKeys(pub String, pub String);

fn main() {
    let my_keys = SyncKeys("Hello, ".to_string(), "world".to_string());
    let c_keys = my_keys.into_ffi_value();
}

#[repr(C)]
#[derive(Debug)]
pub struct SyncKeysC {
    pub sync_key: *mut c_char,
    pub xcs: *mut c_char,
}

#[no_mangle]
pub extern fn ffi_test() -> SyncKeysC {
    let sk = SyncKeys("ffi".to_string(), "test".to_string());
    let c_struct = sk.into_ffi_value();
    //print in same format
    println!("On Rust side, sending the values: sync_key {:?} xcs {:?}", c_struct.sync_key, c_struct.xcs);
    c_struct
}

unsafe impl IntoFfi for SyncKeys {
    type Value = SyncKeysC;
    #[inline]
    fn ffi_default() -> SyncKeysC {
        SyncKeysC {
            sync_key: ptr::null_mut(),
             xcs: ptr::null_mut(),
        }
    }

     #[inline]
     fn into_ffi_value(self) -> SyncKeysC {
         SyncKeysC {
             sync_key: ffi_support::rust_string_to_c(self.0),
             xcs:      ffi_support::rust_string_to_c(self.1),
        }
     }
}
