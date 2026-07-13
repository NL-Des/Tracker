/// Sélectionne l'implémentation OS-specific à exécuter parmi les 3 plateformes
/// supportées, avec une valeur de repli pour les autres. Les expressions sont
/// passées explicitement par l'appelant (pas de résolution de `linux::collect()`
/// par hygiène de macro) pour rester simple et éviter tout piège de portée.
macro_rules! dispatch_os {
    ($linux:expr, $macos:expr, $windows:expr, $fallback:expr) => {{
        #[cfg(target_os = "linux")]
        {
            $linux
        }
        #[cfg(target_os = "macos")]
        {
            $macos
        }
        #[cfg(target_os = "windows")]
        {
            $windows
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            $fallback
        }
    }};
}

pub(crate) use dispatch_os;
