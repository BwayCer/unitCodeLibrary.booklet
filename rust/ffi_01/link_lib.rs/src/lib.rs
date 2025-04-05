#[cfg(feature = "no-mangle")]
mod pub_no_mangle_c_type;
#[cfg(feature = "safer-ffi")]
mod pub_c_type;

mod print_message;
mod utils;

#[cfg(feature = "no-mangle")]
mod lib_no_mangle;
#[cfg(feature = "safer-ffi")]
mod lib_safer_ffi;
