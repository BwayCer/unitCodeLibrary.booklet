use safer_ffi::prelude::{c_slice, char_p};
use std::ffi::{CStr, CString};

use std::io::{self, Write};
use std::process;
use std::thread::sleep;
use std::time::Duration;

use clap::{ArgAction, Parser};
use sysinfo::System;

mod internal_data;
mod lib_safer_ffi;
mod print_message;
mod pub_c_type;
mod utils;
use lib_safer_ffi::*;
use pub_c_type::{LaunchEvent, PureDataArgs};
use utils::RandomValue;

#[derive(Parser, Debug)]
#[command(name = "generate_headers")]
struct InputArgs {
    #[arg(short, long, action = ArgAction::SetTrue, help = "查看記憶體變化.")]
    test: bool,

    #[arg(short, long, action = ArgAction::SetTrue, help = "沒有釋放記憶體.")]
    leak: bool,

    #[arg(
        long,
        action = ArgAction::SetTrue,
        help = "當使用 `--test`, `--leak` 時可以強制打印過程訊息."
    )]
    trace: bool,
}

fn main() {
    let args = &InputArgs::parse();
    let is_trace = args.trace;

    if args.leak {
        test_memory_leak(is_trace, true);
    } else if args.test {
        test_memory_leak(is_trace, false);
    } else {
        normal_operation();
    }
}

fn normal_operation() {
    let is_trace = true;
    let is_leak = false;

    call_base_func();
    call_pure_data(is_trace);
    call_concat_string(is_trace, is_leak);
    call_concat_array(is_trace, is_leak);

    call_register_callback(is_trace, is_leak, 1);
    call_trigger_callback(is_trace);
    unregister_callback(is_trace);
    call_register_callback(is_trace, is_leak, 2);
    call_trigger_callback(is_trace);
    unregister_callback(is_trace);

    for _ in 0..3 {
        call_static_counter(is_trace);
        call_static_data(is_trace);
        close_static(is_trace);
    }
}

fn test_memory_leak(is_trace: bool, is_leak: bool) {
    let process_pid = process::id();

    let sleep_dur = Duration::from_millis(5000);
    let call_interval_sleep_dur = Duration::from_millis(700);
    let try_count = 50000;
    let call_interval_count = 2500;

    let first_usage = read_memory_usage(true, process_pid, 0, 0);
    let mut last_usage = first_usage;

    call_base_func();
    println!("---");
    last_usage = read_memory_usage(false, process_pid, last_usage, first_usage);
    to_sleep(Duration::from_secs(3));

    println!("\n---\ntest: call_pure_data");
    for i in 0..try_count {
        call_pure_data(is_trace);
        if i % call_interval_count == 0 && i != 0 {
            read_memory_usage(is_trace, process_pid, last_usage, first_usage);
            to_sleep(call_interval_sleep_dur);
        }
    }
    last_usage = read_memory_usage(is_trace, process_pid, last_usage, first_usage);
    to_sleep(sleep_dur);

    println!("\n---\ntest: call_concat_string");
    for i in 0..try_count {
        call_concat_string(is_trace, is_leak);
        if i % call_interval_count == 0 && i != 0 {
            read_memory_usage(is_trace, process_pid, last_usage, first_usage);
            to_sleep(call_interval_sleep_dur);
        }
    }
    last_usage = read_memory_usage(is_trace, process_pid, last_usage, first_usage);
    to_sleep(sleep_dur);

    println!("\n---\ntest: call_concat_array");
    for i in 0..try_count {
        call_concat_array(is_trace, is_leak);
        if i % call_interval_count == 0 && i != 0 {
            read_memory_usage(is_trace, process_pid, last_usage, first_usage);
            to_sleep(call_interval_sleep_dur);
        }
    }
    last_usage = read_memory_usage(is_trace, process_pid, last_usage, first_usage);
    to_sleep(sleep_dur);

    println!("\n---\ntest: call_trigger_callback");
    call_register_callback(is_trace, is_leak, 1);
    for i in 0..try_count {
        call_trigger_callback(is_trace);
        if i % call_interval_count == 0 && i != 0 {
            read_memory_usage(is_trace, process_pid, last_usage, first_usage);
            to_sleep(call_interval_sleep_dur);
        }
    }
    unregister_callback(is_trace);
    last_usage = read_memory_usage(is_trace, process_pid, last_usage, first_usage);
    to_sleep(sleep_dur);

    println!("\n---\ntest: call_register_callback, unregister_callback");
    for i in 0..try_count {
        call_register_callback(is_trace, is_leak, ((i % 2) + 1) as u8);
        call_trigger_callback(is_trace);
        unregister_callback(is_trace);
        if i % call_interval_count == 0 && i != 0 {
            read_memory_usage(is_trace, process_pid, last_usage, first_usage);
            to_sleep(call_interval_sleep_dur);
        }
    }
    last_usage = read_memory_usage(is_trace, process_pid, last_usage, first_usage);
    to_sleep(sleep_dur);

    println!("\n---\ntest: call_static_counter");
    for i in 0..try_count {
        call_static_counter(is_trace);
        if i % call_interval_count == 0 && i != 0 {
            read_memory_usage(is_trace, process_pid, last_usage, first_usage);
            to_sleep(call_interval_sleep_dur);
        }
    }
    last_usage = read_memory_usage(is_trace, process_pid, last_usage, first_usage);
    to_sleep(sleep_dur);

    println!("\n---\ntest: call_static_data");
    for i in 0..try_count {
        call_static_data(is_trace);
        if i % call_interval_count == 0 && i != 0 {
            read_memory_usage(is_trace, process_pid, last_usage, first_usage);
            to_sleep(call_interval_sleep_dur);
        }
    }
    last_usage = read_memory_usage(is_trace, process_pid, last_usage, first_usage);
    to_sleep(sleep_dur);

    println!("\n---\ntest: close_static");
    for i in 0..try_count {
        for _ in 0..11 {
            call_static_counter(is_trace);
            call_static_data(is_trace);
        }
        if !is_leak {
            close_static(is_trace);
        }

        if i % call_interval_count == 0 && i != 0 {
            read_memory_usage(is_trace, process_pid, last_usage, first_usage);
            to_sleep(call_interval_sleep_dur);
        }
    }
    last_usage = read_memory_usage(is_trace, process_pid, last_usage, first_usage);
    to_sleep(sleep_dur);

    println!("\n---\ntest: wait 9 sec");
    for _ in 0..9 {
        to_sleep(Duration::from_secs(1));
        read_memory_usage(is_trace, process_pid, last_usage, first_usage);
    }
}

fn read_memory_usage(is_trace: bool, process_pid: u32, prev_usage: u64, first_usage: u64) -> u64 {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut usage = 0;
    for (pid, process) in sys.processes() {
        if pid.as_u32() == process_pid {
            usage = process.memory() / 1024;
            break;
        }
    }

    if usage == 0 {
        println!("Failed to read memory usage.");
        return prev_usage;
    }
    print!(
        "{}{1}{2}\n{1}",
        if is_trace { "" } else { "\r" },
        if is_trace { "---\n" } else { "" },
        if prev_usage == 0 {
            format!("Memory usage: {} KB", usage)
        } else {
            format!(
                "Memory usage: {} KB (diff: {:+} KB) (diff: {:+} KB with first)",
                usage,
                usage - prev_usage,
                usage - first_usage,
            )
        },
    );

    return usage;
}

fn to_sleep(dur: Duration) {
    print!("休眠 {} 毫秒...", dur.as_millis());
    io::stdout().flush().unwrap();
    sleep(dur);
}

fn call_base_func() {
    print_message::print_base_func_by_call();
    base_func();
}

fn call_pure_data(is_trace: bool) {
    let mut rng = RandomValue::new();

    let argu1 = rng.gen_bool();
    let argu2 = rng.gen_i8();
    let argu3 = PureDataArgs::random_one();
    print_message::print_pure_data_by_call(is_trace, argu1, argu2, &argu3);

    let result = pure_data(is_trace, argu1, argu2, argu3);
    print_message::print_pure_data_by_result(is_trace, &result);
}

fn call_concat_string(is_trace: bool, is_leak: bool) {
    let mut rng = RandomValue::new();

    let txt1 = "Hello";
    let txt2_ = rng.gen_str();
    let txt2 = txt2_.as_str();
    print_message::print_concat_string_by_call(is_trace, txt1, txt2);

    let c_string_1 = CString::new(txt1).unwrap();
    let c_string_2 = CString::new(txt2).unwrap();
    let c_str_1: &CStr = c_string_1.as_c_str();
    let c_str_2: &CStr = c_string_2.as_c_str();
    let argu1 = char_p::Ref::from(c_str_1);
    let argu2 = char_p::Ref::from(c_str_2);

    let result = concat_string(is_trace, argu1, argu2);

    let txt = result.to_str();
    print_message::print_concat_string_by_result(is_trace, txt);

    if !is_leak {
        // free_string(result);
    }
}

fn call_concat_array(is_trace: bool, is_leak: bool) {
    let mut rng = RandomValue::new();

    let vec_i32_1 = rng.gen_array_i32(3);
    let vec_i32_2 = rng.gen_array_i32(4);
    let array_i32_1 = vec_i32_1.as_slice();
    let array_i32_2 = vec_i32_2.as_slice();
    print_message::print_concat_array_by_call(is_trace, array_i32_1, array_i32_2);

    let argu1: c_slice::Ref<i32> = c_slice::Ref::from(array_i32_1);
    let argu2: c_slice::Ref<i32> = c_slice::Ref::from(array_i32_2);

    let result = concat_array(is_trace, argu1, argu2);

    let array = result.as_slice();
    print_message::print_concat_array_by_result(is_trace, array);

    if !is_leak {
        // free_array_i32(result);
    }
}

fn launch_event_base(
    index: u8,
    is_trace: bool,
    is_leak: bool,
    pure_data_args: PureDataArgs,
    char_boxed: char_p::Box,
    array_boxed: c_slice::Box<i32>,
) {
    print_message::print_trigger_callback_by_receive_event(
        is_trace,
        index,
        &pure_data_args,
        char_boxed.to_str(),
        array_boxed.as_slice(),
    );

    if !is_leak {
        free_string(is_trace, char_boxed);
        free_array_i32(is_trace, array_boxed);
    }
}

unsafe extern "C" fn launch_event_01(
    is_trace: bool,
    is_leak: bool,
    pure_data_args: PureDataArgs,
    char_boxed: char_p::Box,
    array_boxed: c_slice::Box<i32>,
) {
    launch_event_base(
        1,
        is_trace,
        is_leak,
        pure_data_args,
        char_boxed,
        array_boxed,
    );
}

unsafe extern "C" fn launch_event_02(
    is_trace: bool,
    is_leak: bool,
    pure_data_args: PureDataArgs,
    char_boxed: char_p::Box,
    array_boxed: c_slice::Box<i32>,
) {
    launch_event_base(
        2,
        is_trace,
        is_leak,
        pure_data_args,
        char_boxed,
        array_boxed,
    );
}

fn call_register_callback(is_trace: bool, is_leak: bool, launch_event_index: u8) {
    let callback: LaunchEvent = if launch_event_index == 2 {
        launch_event_02
    } else {
        launch_event_01
    };
    print_message::print_register_callback_by_call(is_trace, callback);
    register_callback(is_trace, is_leak, callback);
}

fn call_trigger_callback(is_trace: bool) {
    print_message::print_trigger_callback_by_call(is_trace);
    for _ in 0..3 {
        trigger_callback(is_trace);
    }
}

fn call_static_counter(is_trace: bool) {
    print_message::print_static_counter_by_call(is_trace);
    for _ in 0..3 {
        let result = static_counter(is_trace);
        print_message::print_static_counter_by_result(is_trace, result);
    }
}

fn call_static_data(is_trace: bool) {
    print_message::print_static_data_by_call(is_trace);
    for _ in 0..3 {
        static_data(is_trace);
    }
}
