use super::InputDevices;

/// Pas de mécanisme simple équivalent à `system_profiler` pour distinguer
/// souris/manettes sans dépendance IOKit : renvoie des listes vides plutôt
/// qu'une donnée approximative (même choix que pour le clavier macOS).
pub fn collect() -> InputDevices {
    InputDevices {
        mice: Vec::new(),
        gamepads: Vec::new(),
        touchpads: Vec::new(),
    }
}
