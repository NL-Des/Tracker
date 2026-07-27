rust_i18n::i18n!("locales", fallback = "en");

pub mod browsers;
pub mod command;
pub mod consent;
pub mod hardware;
pub mod markdown;
pub mod os_dispatch;
pub mod remote_export;
pub mod report;
pub mod software;
pub mod storage;
pub mod xml;

pub use report::SystemReport;
