use super::ProxyConfigInfo;

/// `scutil --proxy` est en lecture libre. Format :
/// "  HTTPEnable : 1\n  HTTPProxy : proxy.example.com\n  HTTPPort : 8080".
fn parse_value<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        line.trim()
            .strip_prefix(key)?
            .trim_start()
            .strip_prefix(':')
            .map(|v| v.trim())
    })
}

pub fn collect() -> Option<ProxyConfigInfo> {
    let text = crate::command::run("scutil", &["--proxy"])?;

    let http_enabled = parse_value(&text, "HTTPEnable") == Some("1");
    let https_enabled = parse_value(&text, "HTTPSEnable") == Some("1");
    if !http_enabled && !https_enabled {
        return None;
    }

    let http_proxy = http_enabled
        .then(|| match (parse_value(&text, "HTTPProxy"), parse_value(&text, "HTTPPort")) {
            (Some(h), Some(p)) => Some(format!("{h}:{p}")),
            _ => None,
        })
        .flatten();
    let https_proxy = https_enabled
        .then(|| match (parse_value(&text, "HTTPSProxy"), parse_value(&text, "HTTPSPort")) {
            (Some(h), Some(p)) => Some(format!("{h}:{p}")),
            _ => None,
        })
        .flatten();

    Some(ProxyConfigInfo {
        http_proxy,
        https_proxy,
        no_proxy: None,
        source: "scutil".to_string(),
    })
}
