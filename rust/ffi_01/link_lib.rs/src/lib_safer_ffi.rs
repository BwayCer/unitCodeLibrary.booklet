// use safer_ffi::prelude::*;
use safer_ffi::prelude::ffi_export;

use super::print_message;
use super::pub_c_type::{PureDataArgs, PureDataResult};
use super::utils::RandomValue;

impl PureDataResult {
    pub fn random_one() -> Self {
        let mut rng = RandomValue::new();
        PureDataResult {
            rtn_yn: rng.gen_bool(),
            rtn_16: rng.gen_u16(),
            rtn_32: rng.gen_i32(),
            rtn_64: rng.gen_f64(),
            rtn_enum: rng.gen_fruit(),
        }
    }
}

#[ffi_export]
pub fn base_func() {
    print_message::print_base_func_by_send_response("safer-ffi");
}

#[ffi_export]
pub fn pure_data(argu1: bool, argu2: i8, argu3: PureDataArgs) -> PureDataResult {
    print_message::print_pure_data_by_receive_call(argu1, argu2, &argu3);
    let output = PureDataResult::random_one();
    print_message::print_pure_data_by_send_response(&output);
    output
}
