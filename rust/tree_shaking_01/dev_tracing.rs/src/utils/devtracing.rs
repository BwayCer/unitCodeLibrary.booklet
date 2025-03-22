use cfg_if::cfg_if;
use tracing::Level;

// ERROR, WARN, INFO, DEBUG, TRACE
#[allow(dead_code)]
const LOG_LEVEL: Level = Level::DEBUG;

cfg_if! {
    if #[cfg(debug_assertions)] {
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! error {
            ($($arg:tt)*) => {{ ::tracing::error!($($arg)*); }};
        }
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! warn {
            ($($arg:tt)*) => {{ ::tracing::warn!($($arg)*); }};
        }
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! info {
            ($($arg:tt)*) => {{ ::tracing::info!($($arg)*); }};
        }
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! debug {
            ($($arg:tt)*) => {{ ::tracing::debug!($($arg)*); }};
        }
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! trace {
            ($($arg:tt)*) => {{ ::tracing::trace!($($arg)*); }};
        }
    } else {
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! error { ($($arg:tt)*) => (); }
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! warn { ($($arg:tt)*) => (); }
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! info { ($($arg:tt)*) => (); }
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! debug { ($($arg:tt)*) => (); }
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! trace { ($($arg:tt)*) => (); }
    }
}

#[allow(dead_code)]
pub fn builder() {
    #[cfg(debug_assertions)]
    {
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(LOG_LEVEL)
            .init();
    }
}

#[allow(dead_code, unused_variables)]
pub fn fmt(file_path: &str) {
    #[cfg(debug_assertions)]
    {
        let file = std::fs::File::create(file_path).expect(&format!("Unable to create log file: \"{}\"", file_path));
        tracing_subscriber::fmt()
            .with_max_level(LOG_LEVEL)
            .with_writer(std::io::stdout)
            .with_writer(file)
            .init();
    }
}
