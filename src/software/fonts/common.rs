/// `fc-list` (fontconfig) est disponible en lecture libre sur Linux et sur
/// macOS s'il est installé (ex. via Homebrew) ; renvoie une liste vide sinon.
pub fn collect() -> Vec<String> {
    let Some(text) = crate::command::run("fc-list", &[":", "family"]) else {
        return Vec::new();
    };

    text.lines()
        .flat_map(|line| line.split(','))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
