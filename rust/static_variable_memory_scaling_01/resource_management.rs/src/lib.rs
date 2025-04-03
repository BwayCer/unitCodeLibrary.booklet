mod try_mem_management {
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(3);

    fn counter() -> u32 {
        COUNTER.fetch_add(1, Ordering::SeqCst)
    }

    use once_cell::sync::OnceCell;
    use std::sync::Mutex;

    static ARRAY_STRING: OnceCell<Mutex<Vec<String>>> = OnceCell::new();

    pub fn record_event() -> String {
        if let Some(arr_mutex) = ARRAY_STRING.get() {
            let counter_value = counter();

            let mut arr_guard = arr_mutex.lock().unwrap();
            arr_guard.push(counter_value.to_string());

            let result = arr_guard
                .iter()
                .skip(arr_guard.len().saturating_sub(3))
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");

            format!(
                "{}, len: {}, cap: {}",
                result,
                arr_guard.len(),
                arr_guard.capacity()
            )
        } else {
            ARRAY_STRING.set(Mutex::new(vec!["init".to_string()])).ok();
            "init".to_string()
        }
    }

    pub fn static_array_capacity() -> u32 {
        let cap = if let Some(arr_mutex) = ARRAY_STRING.get() {
            let arr_guard = arr_mutex.lock().unwrap();
            arr_guard.capacity()
        } else {
            0
        };
        cap as u32
    }

    pub fn close_static() {
        COUNTER.store(0, Ordering::SeqCst);

        if let Some(arr_mutex) = ARRAY_STRING.get() {
            let mut arr_guard = arr_mutex.lock().unwrap();
            arr_guard.clear();
            arr_guard.shrink_to_fit();
        }
    }
}

use std::io::{self, Write};
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::thread::sleep;
use std::time::Duration;

use cfg_if::cfg_if;
use procfs::process::Process;

use try_mem_management::{close_static, record_event, static_array_capacity};

struct OneRoundInfo {
    run_count: usize,
    run_count_magnitude: i8,
    quantity: i8,
    usage: i64,
    max_usage: i64,
}

struct MemInfo {
    #[cfg(feature = "meminfo-struct")]
    data: Vec<OneRoundInfo>,
}

impl MemInfo {
    fn magnitude(run_count: usize) -> i8 {
        let mag_ = (run_count as f64).log2().ceil() as i8;
        if mag_ <= 12 {
            12
        } else {
            mag_
        }
    }

    fn data_push(data: &mut Vec<OneRoundInfo>, run_count: usize, usage: i64, max_usage: i64) {
        let run_count_magnitude = MemInfo::magnitude(run_count);
        if let Some(info) = data.last_mut() {
            if run_count_magnitude == info.run_count_magnitude
                && usage == info.usage
                && max_usage == info.max_usage
            {
                info.quantity += 1;
                return;
            }
        }
        data.push(OneRoundInfo {
            run_count,
            run_count_magnitude,
            quantity: 1,
            usage,
            max_usage,
        });
    }

    fn data_print(data: &Vec<OneRoundInfo>, curr_usage: i64) {
        println!("Record info:");
        let mut prev_usage = 0;
        for idx in 0..data.len() {
            let OneRoundInfo {
                run_count: the_run_count,
                run_count_magnitude: the_run_count_magnitude,
                quantity: the_quantity,
                usage: the_usage,
                max_usage: the_max_usage,
            } = data[idx];

            println!(
                "  run{} {:>9}: {:>5} KB (diff: {:>+5} KB), Max: {:>5} KB (diff: {:>+5} KB)",
                if the_usage < prev_usage { "*" } else { " " },
                if the_quantity == 1 {
                    the_run_count.to_string()
                } else {
                    format!("2^{} x {}", the_run_count_magnitude, the_quantity)
                },
                the_usage,
                the_usage - curr_usage,
                the_max_usage,
                the_max_usage - curr_usage,
            );

            prev_usage = the_usage;
        }
    }
}

cfg_if! {
    if #[cfg(feature = "meminfo-struct")] {
        pub const PLAN_RUN_COUNT_LIST_LENGTH: u8 = 28;

        const PLAN_RUN_COUNT_LIST: [u32; PLAN_RUN_COUNT_LIST_LENGTH as usize] = [
           3, 129, 513, 1025, 65537, 100000, 10000, 10000, 10000, //
           32769, 1000, 1000, 100000, 10000, 10000, 10000, 32769, //
           1000, 1000, 100000, 10000, 10000, 32769, 1000, 1000, 100000, 10000, 10000, //
        ];

        impl MemInfo {
            pub fn new(data_cap: u8) -> Self {
                let data = Vec::<OneRoundInfo>::with_capacity(data_cap as usize);
                MemInfo { data }
            }

            pub fn push(&mut self, run_count: usize, usage: i64, max_usage: i64) {
                MemInfo::data_push(&mut self.data, run_count, usage, max_usage);
            }

            fn print(&self, curr_usage: i64) {
                MemInfo::data_print(&self.data, curr_usage);
            }
        }

        static IS_LOCKED_MEM_INFO: AtomicBool = AtomicBool::new(false);
        static MEM_INFO: AtomicPtr<MemInfo> = AtomicPtr::new(ptr::null_mut());

        fn get_mem_info() -> Option<MemInfo> {
            if !IS_LOCKED_MEM_INFO.load(Ordering::SeqCst) {
                let ptr = MEM_INFO.swap(ptr::null_mut(), Ordering::SeqCst);
                if !ptr.is_null() {
                    let the_obj = unsafe { *Box::from_raw(ptr) };
                    IS_LOCKED_MEM_INFO.store(true, Ordering::SeqCst);
                    return Some(the_obj);
                }
            }
            None
        }

        fn unlock_mem_info(mem_info: MemInfo) {
            let ptr = Box::into_raw(Box::new(mem_info));
            MEM_INFO.store(ptr, Ordering::SeqCst);
            IS_LOCKED_MEM_INFO.store(false, Ordering::SeqCst);
        }
    } else {
        pub const PLAN_RUN_COUNT_LIST_LENGTH: u8 = 40;

        const PLAN_RUN_COUNT_LIST: [u32; PLAN_RUN_COUNT_LIST_LENGTH as usize] = [
           3, 129, 513, 1025, 65537, 100000, 10000, 10000, 10000, //
           32769, 1000, 1000, 100000, 10000, 10000, 10000, 32769, //
           1000, 1000, 100000, 10000, 10000, 10000, 32769, //
           3, 129, 513, 1025, 65537, 100000, 10000, 10000, //
           3, 129, 513, 1025, 65537, 100000, 10000, 10000, //
        ];

        static IS_LOCKED_ALL_INFO: AtomicBool = AtomicBool::new(false);
        static ALL_INFO: AtomicPtr<Vec<OneRoundInfo>> = AtomicPtr::new(ptr::null_mut());

        fn get_mem_info() -> Option<Vec<OneRoundInfo>> {
            if !IS_LOCKED_ALL_INFO.load(Ordering::SeqCst) {
                let ptr = ALL_INFO.swap(ptr::null_mut(), Ordering::SeqCst);
                if !ptr.is_null() {
                    let the_obj = unsafe { *Box::from_raw(ptr) };
                    IS_LOCKED_ALL_INFO.store(true, Ordering::SeqCst);
                    return Some(the_obj);
                }
            }
            None
        }

        fn unlock_mem_info(all_info: Vec<OneRoundInfo>) {
            let ptr = Box::into_raw(Box::new(all_info));
            ALL_INFO.store(ptr, Ordering::SeqCst);
            IS_LOCKED_ALL_INFO.store(false, Ordering::SeqCst);
        }
    }
}

static IS_INIT_MEM_INFO: AtomicBool = AtomicBool::new(false);

fn init_mem_info() {
    if !IS_INIT_MEM_INFO.load(Ordering::SeqCst) {
        // 99 是亂數可能執行的次數
        let mem_info_cap = PLAN_RUN_COUNT_LIST_LENGTH + 99;
        #[cfg(feature = "meminfo-struct")]
        {
            let mem_info = MemInfo::new(mem_info_cap);
            unlock_mem_info(mem_info);
        }
        #[cfg(feature = "meminfo-fn")]
        {
            let mem_info_data = Vec::<OneRoundInfo>::with_capacity(mem_info_cap as usize);
            unlock_mem_info(mem_info_data);
        }
        IS_INIT_MEM_INFO.store(true, Ordering::SeqCst);
    }
}

fn get_mem_usage() -> i64 {
    let pid = std::process::id();
    let process = Process::new(pid as i32).unwrap();
    (process.stat().unwrap().rss * 4) as i64
}

pub fn run_round_by_plan(plan_idx: u8) {
    let run_count = PLAN_RUN_COUNT_LIST[plan_idx as usize];
    run_round(run_count);
}

pub fn run_round(run_count: u32) {
    let run_count = run_count as usize;

    let get_mem_usage_ = get_mem_usage;
    let get_mem_usage = |max_usage: i64| -> (i64, i64) {
        let curr_usage = get_mem_usage_();
        (
            curr_usage,
            if max_usage < curr_usage {
                curr_usage
            } else {
                max_usage
            },
        )
    };
    let print_mem_usage = |prefex: &str, usage: i64| {
        println!(
            "{}Memory usage: {} KB; array_string cap: {}",
            prefex,
            usage,
            static_array_capacity()
        );
    };

    let mut curr_usage;
    let mut max_usage = 0;
    println!("run start");
    for run_idx in 0..run_count {
        let message = record_event();
        print!("\r>> run message: {}", message);
        io::stdout().flush().unwrap();

        if run_idx % 507 == 1 || run_idx == run_count {
            (_, max_usage) = get_mem_usage(max_usage);
            io::stdout().flush().unwrap();
            sleep(Duration::from_millis(70));
        }
    }
    (curr_usage, max_usage) = get_mem_usage(max_usage);
    print_mem_usage("\n>> ", curr_usage);
    sleep(Duration::from_millis(700));

    println!("close start");
    for idx in 1..=2 {
        close_static();

        let ms = 200 * idx;
        print!(">> sleep {} 毫秒", ms);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(ms));

        (curr_usage, _) = get_mem_usage(max_usage);
        print_mem_usage("; ", curr_usage);
    }

    init_mem_info();
    #[cfg(feature = "meminfo-struct")]
    {
        if let Some(mut mem_info) = get_mem_info() {
            mem_info.push(run_count, curr_usage, max_usage);
            mem_info.print(curr_usage);
            unlock_mem_info(mem_info);
        }
    }
    #[cfg(feature = "meminfo-fn")]
    {
        if let Some(mut mem_info_data) = get_mem_info() {
            MemInfo::data_push(&mut mem_info_data, run_count, curr_usage, max_usage);
            MemInfo::data_print(&mem_info_data, curr_usage);
            unlock_mem_info(mem_info_data);
        }
    }
    println!("---");
}

pub fn check_mem_usage() {
    let curr_usage = get_mem_usage();
    println!("Curr usage: {curr_usage}");
    println!("---");
}
