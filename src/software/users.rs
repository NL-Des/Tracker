use serde::Serialize;
use sysinfo::Users;

#[derive(Serialize)]
pub struct UserAccountInfo {
    pub name: String,
    pub uid: String,
    pub gid: String,
    pub groups: Vec<String>,
}

/// Liste les comptes système (ex: `/etc/passwd` sur Linux), pas les sessions
/// actuellement connectées.
pub fn collect() -> Vec<UserAccountInfo> {
    let users = Users::new_with_refreshed_list();
    users
        .iter()
        .map(|user| UserAccountInfo {
            name: user.name().to_string(),
            uid: format!("{:?}", user.id()),
            gid: format!("{:?}", user.group_id()),
            groups: user.groups().iter().map(|g| g.name().to_string()).collect(),
        })
        .collect()
}
