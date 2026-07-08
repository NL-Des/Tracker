use super::FanInfo;

/// Pas d'API publique/CLI standard pour lire les ventilateurs sur macOS
/// (les outils comme smcFanControl utilisent des appels IOKit privés) :
/// renvoie une liste vide plutôt qu'une donnée approximative, même choix
/// que pour le clavier/les manettes macOS déjà en place.
pub fn collect() -> Vec<FanInfo> {
    Vec::new()
}
