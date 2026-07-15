use super::PowerProfileInfo;

/// `pmset -g` est en lecture libre, pas d'admin requis. macOS n'a pas de
/// notion de "profil d'alimentation" nommé comme GNOME/Windows ; on expose
/// le mode d'hibernation (`hibernatemode`) comme information la plus proche.
pub fn collect() -> PowerProfileInfo {
    let Some(text) = crate::command::run("pmset", &["-g"]) else {
        return PowerProfileInfo { profile: None, sleep_mode: None };
    };

    let sleep_mode = text.lines().find_map(|line| {
        line.trim().strip_prefix("hibernatemode").map(|v| v.trim().to_string())
    });

    PowerProfileInfo { profile: None, sleep_mode }
}
