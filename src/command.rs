use std::process::Command;

/// Exécute une commande et renvoie sa sortie stdout (UTF-8 lossy), uniquement
/// si elle s'est terminée avec un code de sortie de succès.
pub fn run(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Comme `run`, mais ignore le code de sortie : certains outils (ex.
/// `smartctl`) utilisent un code de sortie en bitmask où "non-zéro" ne
/// signifie pas un échec de la commande elle-même.
pub fn run_lenient(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Comme `run`, mais définit des variables d'environnement pour la commande
/// (ex: forcer `LC_ALL=C` pour un parsing fiable indépendant de la locale).
pub fn run_with_env(program: &str, args: &[&str], env: &[(&str, &str)]) -> Option<String> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    for (key, value) in env {
        cmd.env(key, value);
    }
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Exécute une commande et renvoie stdout, ou stderr si stdout est vide
/// (certains outils, ex. `java --version`, écrivent sur stderr selon la
/// version). Échoue si le code de sortie n'est pas un succès.
pub fn run_stdout_or_stderr(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        Some(stdout.into_owned())
    } else {
        Some(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

/// Comme `run_stdout_or_stderr`, mais ignore le code de sortie.
pub fn run_lenient_stdout_or_stderr(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.trim().is_empty() {
        Some(stdout.into_owned())
    } else {
        Some(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}
