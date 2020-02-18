use ffi_support::{self, IntoFfi};
pub struct SyncKeys(pub i8, pub i8);

#[repr(C)]
#[derive(Debug)]
pub struct SyncKeysC {
    pub sync_key: i8,
    pub xcs: i8,
}

#[no_mangle]
pub extern fn ffi_test() -> SyncKeysC {
    let sk = SyncKeys(6, 7);
    let c_struct = sk.into_ffi_value();
    println!("On Rust side, sending the values: sync_key {:?} xcs {:?}", c_struct.sync_key, c_struct.xcs);
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
