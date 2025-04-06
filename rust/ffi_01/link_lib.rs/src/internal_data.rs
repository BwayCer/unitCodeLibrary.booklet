use std::collections::VecDeque;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, Ordering};

use super::pub_c_type::LaunchEvent;

struct EventLog {
    event_list: VecDeque<String>,
}

impl EventLog {
    fn new() -> Self {
        let mut event_list = VecDeque::with_capacity(3);
        event_list.push_back("init".to_string());
        EventLog { event_list }
    }

    fn record(&mut self, name: &str) {
        let event_list_len = self.event_list.len();
        if event_list_len == 3 {
            self.event_list.pop_front();
        }
        self.event_list.push_back(name.to_string());
    }

    fn message(&self) -> String {
        self.event_list
            .iter()
            .map(|item| item.as_str())
            .collect::<Vec<&str>>()
            .join(" -> ")
    }
}

static COUNTER: AtomicU32 = AtomicU32::new(3);
static EVENT_LOG: AtomicPtr<EventLog> = AtomicPtr::new(ptr::null_mut());

pub fn counter() -> u32 {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

pub fn record_event() -> String {
    let event_log_ptr = EVENT_LOG.swap(ptr::null_mut(), Ordering::SeqCst);
    let mut event_log = if event_log_ptr.is_null() {
        EventLog::new()
    } else {
        unsafe { *Box::from_raw(event_log_ptr) }
    };

    let counter_value = counter();
    event_log.record(&counter_value.to_string());
    let event_message = event_log.message();

    let event_log_boxed = Box::new(event_log);
    let ptr = Box::into_raw(event_log_boxed);
    EVENT_LOG.store(ptr, Ordering::SeqCst);

    event_message
}

pub fn close_static() {
    COUNTER.store(0, Ordering::SeqCst);

    let event_log_ptr = EVENT_LOG.swap(ptr::null_mut(), Ordering::SeqCst);
    if !event_log_ptr.is_null() {
        let _ = unsafe { *Box::from_raw(event_log_ptr) };
    }
}

static IS_LEAK: AtomicBool = AtomicBool::new(false);
static mut LAUNCH_EVENT_OPTION: Option<LaunchEvent> = None;

pub fn set_leak(is_leak: bool) {
    IS_LEAK.store(is_leak, Ordering::SeqCst);
}

pub fn get_leak() -> bool {
    IS_LEAK.load(Ordering::SeqCst)
}

pub fn set_callback(launch_event: LaunchEvent) {
    unsafe {
        LAUNCH_EVENT_OPTION = Some(launch_event);
    }
}

pub fn get_callback() -> Option<LaunchEvent> {
    unsafe {
        if let Some(launch_event) = LAUNCH_EVENT_OPTION {
            return Some(launch_event);
        }
    }
    None
}

pub fn forget_callback() -> bool {
    unsafe {
        if let Some(_) = LAUNCH_EVENT_OPTION {
            LAUNCH_EVENT_OPTION = None;
            return true;
        }
    }
    false
}
