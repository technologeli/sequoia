use tracing_subscriber;

// https://github.com/rust-lang/rust/issues/35853
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

macro_rules! create_log {
    ($log_name:ident) => {
        with_dollar_sign! {
            ($d:tt) => {
                #[macro_export]
                macro_rules! $log_name {
                    ($d($d x:tt)*) => {
                        {
                            tracing::$log_name!($d($d x)*);
                        }
                    }
                }
            }
        }
    };
}

create_log!(trace);
create_log!(debug);
create_log!(info);
create_log!(warn);
create_log!(error);

pub fn init() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::metadata::LevelFilter::DEBUG)
        .init();
    debug!("Initialized log");
}

// they look like this
// #[macro_export]
// macro_rules! info {
//     ($($x:tt)*) => {
//         {
//             tracing::info!($($x)*);
//         }
//     }
// }
