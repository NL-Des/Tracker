use super::PowerProfileInfo;

/// `powerprofilesctl get` (démon `power-profiles-daemon`, GNOME/freedesktop)
/// est en lecture libre, absent sur certaines distributions. Le mode de
/// veille actif est lu depuis `/sys/power/mem_sleep` (lecture libre),
/// repéré entre crochets (ex: "[s2idle] deep").
pub fn collect() -> PowerProfileInfo {
    let profile = crate::command::run("powerprofilesctl", &["get"]).map(|s| s.trim().to_string());

    let sleep_mode = std::fs::read_to_string("/sys/power/mem_sleep").ok().and_then(|s| {
        s.split_whitespace()
            .find(|w| w.starts_with('[') && w.ends_with(']'))
            .map(|w| w.trim_matches(['[', ']']).to_string())
    });

    PowerProfileInfo { profile, sleep_mode }
}
