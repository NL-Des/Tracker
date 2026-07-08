use super::{UpdateHistoryEntryInfo, MAX_ENTRIES};
use serde::Deserialize;
use wmi::WMIConnection;

/// `Win32_QuickFixEngineering` liste les correctifs (patchs) installés,
/// lecture seule via WMI, aucune élévation requise.
#[derive(Deserialize)]
#[serde(rename = "Win32_QuickFixEngineering")]
struct QuickFix {
    #[serde(rename = "HotFixID")]
    hotfix_id: Option<String>,
    #[serde(rename = "InstalledOn")]
    installed_on: Option<String>,
}

pub fn collect() -> Vec<UpdateHistoryEntryInfo> {
    let Ok(con) = WMIConnection::new() else {
        return Vec::new();
    };
    let Ok(fixes) = con.query::<QuickFix>() else {
        return Vec::new();
    };

    let mut entries: Vec<UpdateHistoryEntryInfo> = fixes
        .into_iter()
        .filter_map(|f| {
            f.hotfix_id.map(|id| UpdateHistoryEntryInfo {
                date: f.installed_on.unwrap_or_else(|| "?".to_string()),
                description: id,
            })
        })
        .collect();

    entries.truncate(MAX_ENTRIES);
    entries
}
