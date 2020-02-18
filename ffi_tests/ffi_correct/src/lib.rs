use ffi_support::{self, IntoFfi};
pub struct SyncKeys(pub i32, pub i32);

#[repr(C)]
#[derive(Debug)]
pub struct SyncKeysC {
    pub sync_key: i32,
    pub xcs: i32,
}

#[no_mangle]
pub extern fn ffi_test() -> SyncKeysC {
    let sk = SyncKeys(6, 7);
    let c_struct = sk.into_ffi_value();
    println!("On Rust side, sending the values: {:?}", c_struct);
    c_struct
}

unsafe impl IntoFfi for SyncKeys {
    type Value = SyncKeysC;
    #[inline]
    fn ffi_default() -> SyncKeysC {
        SyncKeysC {
            sync_key: 0,
            xcs: 0,
        }
    }

     #[inline]
     fn into_ffi_value(self) -> SyncKeysC {
         SyncKeysC {
	     sync_key: self.0,
	     xcs: self.1
        }
     }
}
