use safer_ffi::prelude::c_slice;

use super::pub_c_type::{LaunchEvent, PureDataArgs, PureDataResult};

#[cfg(feature = "bin-main")]
pub fn print_base_func_by_call() {
    println!("Lang.rs: base_func: This function is called from a Rust application.");
}

pub fn print_base_func_by_send_response() {
    println!(
        "Lang.rs: base_func: This function is implemented in Rust with \"safer-ffi\" feature and exposed via a DLL!"
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
pub fn print_pure_data_by_call(is_trace: bool, argu1: bool, argu2: i8, argu3: &PureDataArgs) {
    if is_trace {
        println!(
            "Lang.rs: pure_data: call: {}, {}, {}",
            argu1,
            argu2,
            pure_data_args_message(argu3)
        );
    }
}

pub fn print_pure_data_by_receive_call(
    is_trace: bool,
    argu1: bool,
    argu2: i8,
    argu3: &PureDataArgs,
) {
    if is_trace {
        println!(
            "Lang.rs: pure_data: receive call: {}, {}, {}",
            argu1,
            argu2,
            pure_data_args_message(argu3)
        );
    }
}

pub fn print_pure_data_by_send_response(is_trace: bool, result: &PureDataResult) {
    if is_trace {
        println!(
            "Lang.rs: pure_data: send response: {}",
            pure_data_result_message(result),
        );
    }
}

#[cfg(feature = "bin-main")]
pub fn print_pure_data_by_result(is_trace: bool, result: &PureDataResult) {
    if is_trace {
        println!(
            "Lang.rs: pure_data: result: {}",
            pure_data_result_message(result),
        );
    }
}

#[cfg(feature = "bin-main")]
pub fn print_concat_string_by_call(is_trace: bool, argu1: &str, argu2: &str) {
    if is_trace {
        println!("Lang.rs: concat_string: call: {argu1}, {argu2}");
    }
}

pub fn print_concat_string_by_receive_call(is_trace: bool, argu1: &str, argu2: &str) {
    if is_trace {
        println!("Lang.rs: concat_string: receive call: {argu1}, {argu2}");
    }
}

pub fn print_concat_string_by_send_response(is_trace: bool, result: &str) {
    if is_trace {
        println!("Lang.rs: concat_string: send response: {result}");
    }
}

#[cfg(feature = "bin-main")]
pub fn print_concat_string_by_result(is_trace: bool, result: &str) {
    if is_trace {
        println!("Lang.rs: concat_string: result: {result}");
    }
}

pub fn print_free_string(is_trace: bool, value: &str) {
    if is_trace {
        println!("Lang.rs: free_string: {value}");
    }
}

#[cfg(feature = "bin-main")]
pub fn print_concat_array_by_call(is_trace: bool, argu1: &[i32], argu2: &[i32]) {
    if is_trace {
        println!("Lang.rs: concat_array: call: {argu1:?}, {argu2:?}");
    }
}

pub fn print_concat_array_by_receive_call(is_trace: bool, argu1: &[i32], argu2: &[i32]) {
    if is_trace {
        println!("Lang.rs: concat_array: receive call: {argu1:?}, {argu2:?}");
    }
}

pub fn print_concat_array_by_send_response(is_trace: bool, result: &[i32]) {
    if is_trace {
        println!("Lang.rs: concat_array: send response: {result:?}");
    }
}

#[cfg(feature = "bin-main")]
pub fn print_concat_array_by_result(is_trace: bool, result: &[i32]) {
    if is_trace {
        println!("Lang.rs: concat_array: result: {result:?}");
    }
}

pub fn print_free_array_i32(is_trace: bool, value: &[i32], c_slice_boxed: c_slice::Box<i32>) {
    if is_trace {
        println!("Lang.rs: free_array_i32: info: c_slice::Box: ({c_slice_boxed:?})");
        println!("Lang.rs: free_array_i32: main: {value:?}");
    }
}

#[cfg(feature = "bin-main")]
pub fn print_register_callback_by_call(is_trace: bool, launch_event: LaunchEvent) {
    if is_trace {
        println!(
            "Lang.rs: register_callback: call: callback() (ptr:{:p})",
            launch_event
        );
    }
}

pub fn print_register_callback_by_receive_call(is_trace: bool, launch_event: LaunchEvent) {
    if is_trace {
        println!(
            "Lang.rs: register_callback: receive call: callback() (ptr:{:p})",
            launch_event
        );
    }
}

pub fn print_unregister_callback_by_send_response(is_trace: bool, is_forget: bool) {
    if is_trace {
        println!("Lang.rs: unregister_callback: send response: {is_forget}");
    }
}

#[cfg(feature = "bin-main")]
pub fn print_trigger_callback_by_call(is_trace: bool) {
    if is_trace {
        println!("Lang.rs: trigger_callback: call");
    }
}

pub fn print_trigger_callback_by_receive_call(
    is_trace: bool,
    launch_event_option: Option<LaunchEvent>,
) {
    if is_trace {
        print!("Lang.rs: trigger_callback: receive call: ");
        if let Some(launch_event) = launch_event_option {
            println!("callback() (ptr:{:p})", launch_event);
        } else {
            println!("not found callback()");
        }
    }
}

pub fn print_trigger_callback_by_launch_event(
    is_trace: bool,
    pure_data_args: &PureDataArgs,
    text: &str,
    array: &[i32],
) {
    if is_trace {
        println!(
            "Lang.rs: trigger_callback: launch event:\n  {},\n  {},\n  {:?}",
            pure_data_args_message(pure_data_args),
            text,
            array
        );
    }
}

#[cfg(feature = "bin-main")]
pub fn print_trigger_callback_by_receive_event(
    is_trace: bool,
    index: u8,
    pure_data_args: &PureDataArgs,
    text: &str,
    array: &[i32],
) {
    if is_trace {
        println!(
            "Lang.rs: trigger_callback: receive event (0{}):\n  {},\n  {},\n  {:?}",
            index,
            pure_data_args_message(pure_data_args),
            text,
            array
        );
    }
}

#[cfg(feature = "bin-main")]
pub fn print_static_counter_by_call(is_trace: bool) {
    if is_trace {
        println!("Lang.rs: static_counter: call");
    }
}

pub fn print_static_counter_by_send_response(is_trace: bool, value: u32) {
    if is_trace {
        println!("Lang.rs: static_counter: send response: {value}");
    }
}

#[cfg(feature = "bin-main")]
pub fn print_static_counter_by_result(is_trace: bool, result: u32) {
    if is_trace {
        println!("Lang.rs: static_counter: result: {result}");
    }
}

#[cfg(feature = "bin-main")]
pub fn print_static_data_by_call(is_trace: bool) {
    if is_trace {
        println!("Lang.rs: static_data: call");
    }
}

pub fn print_static_data_by_internal_response(is_trace: bool, value: String) {
    if is_trace {
        println!("Lang.rs: static_data: internal response: {}", value);
    }
}

pub fn print_close_static(is_trace: bool) {
    if is_trace {
        println!("Lang.rs: close_static");
    }
}
