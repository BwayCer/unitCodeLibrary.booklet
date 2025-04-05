#[cfg(feature = "no-mangle")]
use super::pub_no_mangle_c_type::{PureDataArgs, PureDataResult};
#[cfg(feature = "safer-ffi")]
use super::pub_c_type::{PureDataArgs, PureDataResult};

#[cfg(feature = "bin-main")]
pub fn print_base_func_by_call() {
    println!("Lang.rs: base_func: This function is called from a Rust application.");
}

pub fn print_base_func_by_send_response(mod_type: &str) {
    println!(
        "Lang.rs: base_func: This function is implemented in Rust with \"{}\" feature and exposed via a DLL!",
        mod_type
    );
}

fn pure_data_args_message(pure_data_args: &PureDataArgs) -> String {
    format!(
        "PureDataArgs {{ {}, {}, {}, {} }}",
        pure_data_args.argu_16,
        pure_data_args.argu_32,
        pure_data_args.argu_64,
        pure_data_args.argu_enum.clone() as u8,
    )
}

fn pure_data_result_message(result: &PureDataResult) -> String {
    let PureDataResult {
        rtn_yn,
        rtn_16,
        rtn_32,
        rtn_64,
        rtn_enum,
    } = result;
    format!(
        "{}, {}, {}, {}, {}",
        rtn_yn,
        rtn_16,
        rtn_32,
        rtn_64,
        rtn_enum.clone() as u8,
    )
}

#[cfg(feature = "bin-main")]
pub fn print_pure_data_by_call(argu1: bool, argu2: i8, argu3: &PureDataArgs) {
    println!(
        "Lang.rs: pure_data: call: {}, {}, {}",
        argu1,
        argu2,
        pure_data_args_message(argu3)
    );
}

pub fn print_pure_data_by_receive_call(argu1: bool, argu2: i8, argu3: &PureDataArgs) {
    println!(
        "Lang.rs: pure_data: receive call: {}, {}, {}",
        argu1,
        argu2,
        pure_data_args_message(argu3)
    );
}

pub fn print_pure_data_by_send_response(result: &PureDataResult) {
    println!(
        "Lang.rs: pure_data: send response: {}",
        pure_data_result_message(result),
    );
}

#[cfg(feature = "bin-main")]
pub fn print_pure_data_by_result(result: &PureDataResult) {
    println!(
        "Lang.rs: pure_data: result: {}",
        pure_data_result_message(result),
    );
}
