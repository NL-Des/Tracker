use serde::Serialize;
use sysinfo::Users;

#[derive(Serialize)]
pub struct UserAccountInfo {
    pub name: String,
    pub uid: String,
    pub gid: String,
    pub groups: Vec<String>,
    /// Dérivé des groupes déjà collectés (`sudo`/`wheel`/`admin` sur
    /// Linux/macOS), ou de `net localgroup administrators` sur Windows —
    /// aucune commande/lecture supplémentaire n'est nécessaire sur
    /// Linux/macOS, l'information est déjà présente dans `groups`.
    pub is_admin: bool,
}

/// Noms de groupes considérés comme donnant des privilèges administrateur
/// sur Linux/macOS.
const ADMIN_GROUPS: &[&str] = &["sudo", "wheel", "admin"];

fn is_admin_by_groups(groups: &[String]) -> bool {
    groups
        .iter()
        .any(|g| ADMIN_GROUPS.contains(&g.to_lowercase().as_str()))
}

/// Parse la sortie de `net localgroup administrators` (pas d'admin requis
/// pour cette lecture) : noms de comptes entre la ligne de tirets et le
/// message de fin.
#[cfg(target_os = "windows")]
fn windows_admin_names() -> Vec<String> {
    let Some(text) = crate::command::run("net", &["localgroup", "administrators"]) else {
        return Vec::new();
    };
    let mut in_members = false;
    text.lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.starts_with("----") {
                in_members = true;
                return None;
            }
            if !in_members || line.is_empty() || line.starts_with("The command completed") {
                return None;
            }
            Some(line.to_string())
        })
        .collect()
}

#[cfg(not(target_os = "windows"))]
fn windows_admin_names() -> Vec<String> {
    Vec::new()
}

/// Liste les comptes système (ex: `/etc/passwd` sur Linux), pas les sessions
/// actuellement connectées.
pub fn collect() -> Vec<UserAccountInfo> {
    let users = Users::new_with_refreshed_list();
    let windows_admins = windows_admin_names();
    users
        .iter()
        .map(|user| {
            let groups: Vec<String> =
                user.groups().iter().map(|g| g.name().to_string()).collect();
            let is_admin = is_admin_by_groups(&groups)
                || windows_admins.iter().any(|n| n.eq_ignore_ascii_case(user.name()));
            UserAccountInfo {
                name: user.name().to_string(),
                uid: format!("{:?}", user.id()),
                gid: format!("{:?}", user.group_id()),
                groups,
                is_admin,
            }
        })
        .collect()
}
