use safer_ffi::prelude::ffi_export;

#[ffi_export]
pub fn my_func_base() {
    println!("This is the Base function in the DLL!");
}
