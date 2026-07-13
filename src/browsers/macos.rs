use super::BrowserInfo;
use std::path::Path;

const KNOWN_BUNDLES: &[(&str, &str)] = &[
    ("Google Chrome", "Google Chrome.app"),
    ("Mozilla Firefox", "Firefox.app"),
    ("Safari", "Safari.app"),
    ("Microsoft Edge", "Microsoft Edge.app"),
    ("Brave", "Brave Browser.app"),
    ("Opera", "Opera.app"),
    ("Vivaldi", "Vivaldi.app"),
];

const APPLICATIONS_DIRS: &[&str] = &["/Applications", "/System/Applications"];

fn bundle_version(bundle_path: &Path) -> Option<String> {
    let plist_path = bundle_path.join("Contents/Info.plist").to_string_lossy().into_owned();
    let text = crate::command::run(
        "plutil",
        &["-extract", "CFBundleShortVersionString", "raw", "-o", "-", &plist_path],
    )?;
    let text = text.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

/// Best-effort : LaunchServices n'expose pas d'API en ligne de commande
/// fiable, on retombe sur une lecture du plist de préférences.
fn default_browser_bundle_id() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let plist = format!(
        "{home}/Library/Preferences/com.apple.LaunchServices/com.apple.launchservices.secure.plist"
    );
    let text = crate::command::run("plutil", &["-convert", "json", "-o", "-", &plist])?;
    let json: serde_json::Value = serde_json::from_str(&text).ok()?;
    let handlers = json.get("LSHandlers")?.as_array()?;
    handlers.iter().find_map(|handler| {
        let scheme = handler.get("LSHandlerURLScheme")?.as_str()?;
        if scheme.eq_ignore_ascii_case("http") {
            handler
                .get("LSHandlerRoleAll")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        } else {
            None
        }
    })
}

pub fn collect() -> Vec<BrowserInfo> {
    let default_bundle_id = default_browser_bundle_id();

    KNOWN_BUNDLES
        .iter()
        .filter_map(|(display_name, bundle_dir)| {
            APPLICATIONS_DIRS.iter().find_map(|apps_dir| {
                let bundle_path = Path::new(apps_dir).join(bundle_dir);
                if !bundle_path.exists() {
                    return None;
                }
                let version = bundle_version(&bundle_path);
                let normalized_name = display_name.to_lowercase().replace(' ', "");
                let is_default = default_bundle_id
                    .as_deref()
                    .map(|id| id.to_lowercase().contains(&normalized_name))
                    .unwrap_or(false);
                Some(BrowserInfo {
                    name: display_name.to_string(),
                    version,
                    path: Some(bundle_path.to_string_lossy().to_string()),
                    is_default,
                    extensions: None,
                })
            })
        })
        .collect()
}
