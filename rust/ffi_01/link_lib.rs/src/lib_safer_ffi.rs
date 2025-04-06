// use safer_ffi::prelude::*;
use safer_ffi::prelude::{c_slice, char_p, ffi_export};

use super::internal_data::{self, forget_callback, get_callback, get_leak, set_callback, set_leak};
use super::print_message;
use super::pub_c_type::{LaunchEvent, PureDataArgs, PureDataResult};
use super::utils::{mix_array, RandomValue};

impl PureDataArgs {
    pub fn random_one() -> Self {
        let mut rng = RandomValue::new();
        PureDataArgs {
            argu_16: rng.gen_u16(),
            argu_32: rng.gen_i32(),
            argu_64: rng.gen_f64(),
            argu_enum: rng.gen_fruit(),
        }
    }
}

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
    print_message::print_base_func_by_send_response();
}

#[ffi_export]
pub fn pure_data(is_trace: bool, argu1: bool, argu2: i8, argu3: PureDataArgs) -> PureDataResult {
    if is_trace {
        print_message::print_pure_data_by_receive_call(is_trace, argu1, argu2, &argu3);
    }
    let output = PureDataResult::random_one();
    if is_trace {
        print_message::print_pure_data_by_send_response(is_trace, &output);
    }
    output
}

#[ffi_export]
pub fn concat_string(
    is_trace: bool,
    argu1: char_p::Ref<'_>,
    argu2: char_p::Ref<'_>,
) -> char_p::Box {
    let input_txt1 = argu1.to_str();
    let input_txt2 = argu2.to_str();
    print_message::print_concat_string_by_receive_call(is_trace, input_txt1, input_txt2);

    let output = format!("{input_txt1} {input_txt2}!");
    print_message::print_concat_string_by_send_response(is_trace, output.as_str());

    output.try_into().unwrap()
}

#[ffi_export]
pub fn free_string(is_trace: bool, char_boxed: char_p::Box) {
    // drop(char_boxed)
    // or
    let txt = char_boxed.to_str();
    print_message::print_free_string(is_trace, txt);
}

#[ffi_export]
pub fn concat_array<'s>(
    is_trace: bool,
    argu1: c_slice::Ref<'s, i32>,
    argu2: c_slice::Ref<'s, i32>,
) -> c_slice::Box<i32> {
    let input_slice1 = argu1.as_slice();
    let input_slice2 = argu2.as_slice();
    print_message::print_concat_array_by_receive_call(is_trace, input_slice1, input_slice2);

    let output = mix_array(input_slice1, input_slice2);
    let output_slice = output.as_slice();
    print_message::print_concat_array_by_send_response(is_trace, output_slice);

    output.into_boxed_slice().into()
}

#[ffi_export]
pub fn free_array_i32(is_trace: bool, array_boxed: c_slice::Box<i32>) {
    // drop(array_boxed);
    // or
    let c_slice_boxed_clone = array_boxed.clone();
    let arr = array_boxed.as_slice();
    print_message::print_free_array_i32(is_trace, arr, c_slice_boxed_clone);
}

fn call_launch_event(is_trace: bool, launch_event: LaunchEvent) {
    let mut rng = RandomValue::new();
    let argu1 = PureDataArgs::random_one();
    let txt = rng.gen_str();
    let array_i32 = rng.gen_array_i32(5);
    print_message::print_trigger_callback_by_launch_event(
        is_trace,
        &argu1,
        txt.as_str(),
        array_i32.as_slice(),
    );

    let argu2: char_p::Box = txt.try_into().unwrap();
    let argu3 = array_i32.into_boxed_slice().into();

    unsafe {
        launch_event(is_trace, get_leak(), argu1, argu2, argu3);
    }
}

#[ffi_export]
pub fn register_callback(is_trace: bool, is_leak: bool, launch_event: LaunchEvent) {
    print_message::print_register_callback_by_receive_call(is_trace, launch_event);
    set_leak(is_leak);
    set_callback(launch_event);
    call_launch_event(is_trace, launch_event);
}

#[ffi_export]
pub fn unregister_callback(is_trace: bool) {
    set_leak(false);
    let is_forget = forget_callback();
    print_message::print_unregister_callback_by_send_response(is_trace, is_forget);
}

#[ffi_export]
pub fn trigger_callback(is_trace: bool) {
    let launch_event_option = get_callback();
    print_message::print_trigger_callback_by_receive_call(is_trace, launch_event_option);
    if let Some(launch_event) = launch_event_option {
        call_launch_event(is_trace, launch_event);
    }
}

#[ffi_export]
pub fn static_counter(is_trace: bool) -> u32 {
    let counter_value = internal_data::counter();
    print_message::print_static_counter_by_send_response(is_trace, counter_value);
    counter_value
}

#[ffi_export]
pub fn static_data(is_trace: bool) {
    let event_message = internal_data::record_event();
    print_message::print_static_data_by_internal_response(is_trace, event_message);
}

#[ffi_export]
pub fn close_static(is_trace: bool) {
    print_message::print_close_static(is_trace);
    internal_data::close_static();
}
