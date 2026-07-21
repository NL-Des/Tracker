# Plan — Mise en application de la section 5 (bilan_client.md)

## Contexte

`bilan_client.md` documente le cadrage d'un futur client GUI Tauri de consentement/opt-in pour le crate `tracker` (aujourd'hui un CLI Rust qui collecte l'inventaire matériel/logiciel de la machine et l'exporte en JSON/MD/XML). La section 5 de ce document liste 10 étapes de haut niveau, non détaillées. L'objectif de cette session est de préparer un plan d'exécution concret pour ces 10 étapes, sans encore écrire de code.

Décisions prises avec l'utilisateur pour cadrer ce plan :
- Les deux questions ouvertes de la section 4 (authentification locale vs distante, contrainte de délai) restent **volontairement non tranchées** — l'écran d'accueil (étape 3) doit rester un simple point d'ancrage, sans logique d'auth.
- Le plan couvre l'intégralité des 10 étapes (pas de sous-ensemble).
- Frontend Tauri : **Vite + JS vanilla** (pas de framework), cohérent avec l'exigence de légèreté actée en 2.1.

Constats vérifiés dans le code actuel (base factuelle du plan) :
- `Cargo.toml` racine : package `tracker`, édition 2024, pas de `[workspace]`. Dépendances : `display-info`, `serde`+derive, `serde_json`, `starship-battery`, `sysinfo`, et `winreg`/`wmi` (Windows uniquement).
- `src/main.rs` déclare 8 `mod` (`browsers`, `command`, `hardware`, `markdown`, `os_dispatch`, `report`, `software`, `xml`) et se contente d'appeler `SystemReport::collect()` puis `save_json()`/`save_markdown()`/`save_xml()`.
- `src/report.rs` (79 lignes) : `SystemReport { generated_at_unix, tool_version, hardware: HardwareInfo, software: SoftwareInfo, browsers: Vec<BrowserInfo>, collection_warnings: Vec<String> }`.
- `HardwareInfo` (`src/hardware/mod.rs`) : 23 champs. `SoftwareInfo` (`src/software/mod.rs`) : 24 champs. Les deux ne dérivent que `Serialize` (pas `Deserialize`/`Default`).
- `src/markdown.rs` (684 lignes) et `src/xml.rs` (683 lignes) écrivent chaque champ « à la main » (pas de boucle générique sur une liste de champs) — point dimensionnant pour l'étape 9.
- Aucun `tests/`, aucun `#[cfg(test)]`, aucun `src/lib.rs`, aucun `src-tauri/`, aucune CI existante — un split lib/bin ne casse rien.
- `.gitignore` ignore `/target` et les 3 `tracker_report.*`. `.cargo/config.toml` (jobs=2, linker mold sur Linux) s'applique à tout futur workspace sans changement.

## Plan détaillé par étape

### Étape 1 — Restructuration workspace
- Créer `src/lib.rs` : y déplacer les 8 `mod` de `main.rs` en `pub mod`, ré-exporter `pub use report::SystemReport;`.
- `src/main.rs` : remplacer les `mod` par `use tracker::report::SystemReport;`, garder le reste identique.
- `Cargo.toml` racine : ajouter `[workspace] members = ["."]` (puis `"src-tauri"` une fois créé à l'étape 3, pour éviter une erreur "member not found" entre-temps). Sections `[lib]`/`[[bin]]` explicites recommandées pour la lisibilité.
- Point d'attention : `SystemReport::collect()` contient un `sleep` bloquant (`sysinfo::MINIMUM_CPU_UPDATE_INTERVAL`) — à isoler dans un thread dédié/`spawn_blocking` côté Tauri (étape 3), pour ne pas geler la webview.
- Vérification : `cargo build`/`cargo run` à la racine doit produire un comportement CLI strictement identique à avant (3 fichiers `tracker_report.*` générés).

### Étape 2 — Modèle de consentement
- Nouveau module `src/consent.rs` (`pub mod consent;` dans `lib.rs`) avec :
  - `HardwareConsent` (23 `bool`, mêmes noms que `HardwareInfo`) et `SoftwareConsent` (24 `bool`, mêmes noms que `SoftwareInfo`) — la convention de nommage identique est actée comme garde-fou mécanique pour les étapes 6/7/9.
  - `ConsentConfig { version: u32, accepted_at_unix: Option<u64>, hardware: HardwareConsent, software: SoftwareConsent, browsers: bool }` (dérive `Serialize + Deserialize + Clone + Debug + PartialEq + Default`).
  - `enum ConsentPreset { None, Minimum, Medium, Maximum }` + `to_config()`. Composition exacte de "Minimum"/"Moyen" **à valider avec l'utilisateur au moment du développement de l'étape 5** — pas un blocage, juste une décision différée explicitement notée.
  - `config_dir()`/`config_path()`/`load()`/`save()` via la crate `directories` (chemins standards par OS).
- **Risque de dérive des ~47 clés** (identifié en section 3 du bilan) : mitigation retenue = un test de parité (`tests/consent_parity.rs`) qui compare les clés sérialisées de `ConsentConfig::default()` à deux tableaux `HARDWARE_FIELDS`/`SOFTWARE_FIELDS` maintenus en commentaire au-dessus de `HardwareInfo`/`SoftwareInfo`. Solution volontairement simple plutôt qu'une macro générant les deux structs, jugée disproportionnée pour la taille du projet.
- Dépendance à ajouter : `directories = "5"` dans le `Cargo.toml` racine.
- Vérification : `cargo test` (parité + sérialisation par défaut + save/load sur répertoire temporaire injectable).

### Étape 3 — Squelette Tauri (Vite + vanilla JS)
- `src-tauri/` : nouveau crate binaire `tracker-gui` (édition 2024), dépendant de `tracker` via `path = ".."`, plus `tauri` v2 et `tauri-build`. Ajouter `"src-tauri"` aux `members` du workspace.
- `frontend/` (à la racine du repo, séparé de `src-tauri/`) : projet Vite vanilla JS minimal — `index.html`, `src/main.js`, `src/home.js`, `src/api.js` (wrapper `invoke()`).
- Commandes IPC dans `src-tauri/src/commands.rs` : `get_consent() -> ConsentConfig`, `save_consent(config: ConsentConfig)`, `collect_and_export(formats: Vec<String>, output_dir: String) -> Vec<String>` — cette dernière en `async`/`spawn_blocking` à cause du sleep bloquant de `collect()`. Pas de filtrage "np" à ce stade (ajouté à l'étape 9) : on valide d'abord la plomberie IPC bout en bout.
- Écran d'accueil : un bouton d'entrée, routage minimal en JS (pas de vrai routeur), sans logique d'authentification (cf. décision de cadrage).
- `.gitignore` à étendre : `frontend/node_modules/`, `frontend/dist/`.
- Vérification : `cargo tauri dev` ouvre la fenêtre, le bouton déclenche un appel IPC visible en DevTools ; `cargo build --workspace` compile les 3 membres.

### Étape 4 — Plomberie i18n
- `locales/fr.yml` / `locales/en.yml` à la racine (convention `rust-i18n`), `rust_i18n::i18n!("locales", fallback = "en")` dans `lib.rs`.
- Remplacer les chaînes françaises en dur de `collection_warnings` (dans `report.rs`) par des `t!("warnings.xxx")`. Locale positionnée via `rust_i18n::set_locale(...)`, appelée par `main.rs` (CLI) ou par une commande IPC `set_locale` (GUI) avant `collect()`.
- **Scope explicitement limité** à `collection_warnings`, conformément à la section 3 du bilan — les titres de section en dur dans `markdown.rs`/`xml.rs` (~1300 lignes à eux deux) sont **hors périmètre v1**, sauf demande contraire du client.
- Frontend : `frontend/src/locales/{fr,en}.json` + petite fonction `t(key)` maison (pas de lib i18n JS, cohérent avec le choix vanilla), sélecteur de langue sur l'écran d'accueil.
- Dépendance à ajouter : `rust-i18n = "3"`.
- Vérification : test unitaire Rust FR/EN + test de fallback sur clé manquante ; test manuel GUI (changement de langue → `collection_warnings` traduit dans l'export).

### Étape 5 — Onglet "Niveaux globaux"
- Commande IPC `get_preset(name: String) -> ConsentConfig` qui appelle `ConsentPreset::to_config()` côté Rust — la logique des 4 presets reste centralisée dans `src/consent.rs`, le frontend ne fait qu'afficher/appliquer.
- État global partagé entre onglets via un petit module `frontend/src/state.js` (pub/sub maison, pas de framework).
- Décision différée à valider ici avec l'utilisateur : composition exacte de "Minimum"/"Moyen" (cf. étape 2).
- Vérification : chaque preset appliqué doit se refléter dans `get_consent` et visuellement dans les onglets Hardware/Software.

### Étape 6 — Onglet "Hardware"
- Pour éviter de dupliquer les 23 noms de champs entre Rust et JS, une commande IPC `list_hardware_fields()` retourne dynamiquement les clés depuis `serde_json::to_value(&HardwareConsent::default())` — le frontend ne code en dur que les métadonnées de présentation (regroupement en sous-onglets, libellés i18n), jamais les noms de champs eux-mêmes.
- Sous-onglets proposés (regroupement à ajuster) : stockage, réseau physique, capteurs, alimentation, système, affichage, bus, périphériques.
- Sauvegarde via bouton explicite (pas d'écriture à chaque clic), pour un horodatage `accepted_at_unix` propre.
- Vérification : cocher/décocher + sauvegarder, relire via `get_consent` ; vérifier que les 23 clés apparaissent chacune exactement une fois dans l'UI.

### Étape 7 — Onglet "Software"
- Même principe que l'étape 6 avec `list_software_fields()` pour les 24 champs de `SoftwareConsent`.
- `browsers` (hors `HardwareConsent`/`SoftwareConsent` dans le modèle de l'étape 2, car `SystemReport.browsers` est un `Vec<BrowserInfo>` séparé) : affiché comme case à part dans le sous-onglet "applications", sans créer d'onglet dédié pour un seul module.
- Sous-onglets proposés : système, processus & utilisateurs, applications, services & tâches, paquets, conteneurs & VM, réseau logiciel, sécurité.

### Étape 8 — Onglet "Réseau"
- Entrée de navigation avec message "à venir" (i18n), aucune logique de collecte associée — aucun `NetworkConsent` créé tant qu'aucun module de collecte réseau dédié n'existe côté Rust (cohérent avec "pas de contenu de remplissage" en section 3).

### Étape 9 — Filtrage à l'export (indicateur "np")
Le chantier le plus volumineux du plan (les structs ne dérivant que `Serialize`, pas de réécriture en `Field<T>` générique — jugée disproportionnée).
- **JSON** : `SystemReport::to_json_pretty_filtered(&self, consent: &ConsentConfig)` — sérialise en `serde_json::Value`, puis remplace mécaniquement `value["hardware"][clé]`/`value["software"][clé]` par `"np"` pour chaque clé désactivée, via une fonction générique qui introspecte `serde_json::to_value(&consent.hardware)` (pas de 47 `if` en dur).
- **Markdown/XML** : signatures `generate(report, consent)` dans `markdown.rs`/`xml.rs` ; chaque bloc d'écriture par champ (~47 points au total, un par champ Hardware/Software) doit vérifier le flag correspondant avant d'écrire le contenu complet, sinon écrire un placeholder "np". Travail mécanique mais volumineux vu les 1300 lignes concernées — un test paramétré (boucle sur les 47 clés, un module désactivé à la fois) est recommandé pour garantir la couverture complète plutôt qu'une vérification manuelle.
- Le CLI (`main.rs`) reste inchangé — le filtrage n'existe que côté GUI, qui charge le `ConsentConfig` courant avant l'export.
- Vérification : tests unitaires JSON (clé filtrée = "np", clé activée = objet complet) + tests MD/XML par recherche de sous-chaîne ; vérification manuelle des 3 formats exportés depuis la GUI.

### Étape 10 — Tests multiplateformes et empaquetage
- `src-tauri/tauri.conf.json` : configuration `bundle` (identifiant, cibles par OS : deb/AppImage sur Linux, msi/nsis sur Windows, dmg/app sur macOS), icônes via `cargo tauri icon`.
- Documenter (nouveau `docs/BUILD.md`) les dépendances système par OS : WebKitGTK sur Linux (paquets variables selon distribution), WebView2 sur Windows (généralement préinstallé), WKWebView sur macOS (natif, mais signature/notarization à trancher avec le client selon le canal de distribution prévu).
- Proposer une CI GitHub Actions (`.github/workflows/ci.yml`, aucune CI n'existe actuellement) avec matrice Linux/macOS/Windows : `cargo build --workspace`, `cargo test --workspace`, `cargo tauri build`.
- Point d'attention : les appels système du crate (`winreg`/`wmi` sous Windows, `command.rs` qui shell-out vers des utilitaires comme `smartctl`) doivent être re-testés depuis un binaire packagé (permissions différentes d'un run en dev, Gatekeeper macOS pour une app non signée).
- Vérification : `cargo tauri build` réussit sur chaque OS testable, installation du bundle sur une machine "propre" (sans toolchain dev), mesure de la taille finale.

## Séquencement

1 (workspace) → 2 (consentement) → 3 (squelette Tauri) → 4 (i18n) → 5/6/7/8 (les 4 onglets, parallélisables une fois 2+3+4 posés) → 9 (filtrage export) → 10 (tests/empaquetage, dépend de tout le reste).

## Dépendances à ajouter (récapitulatif)
- `Cargo.toml` racine : `directories = "5"` (étape 2), `rust-i18n = "3"` (étape 4).
- `src-tauri/Cargo.toml` (nouveau) : `tauri = "2"`, `tauri-build = "2"`, `tracker = { path = ".." }`, `serde`, `serde_json`.
- `frontend/package.json` (nouveau) : `vite`, `@tauri-apps/api`, `@tauri-apps/cli`.

## Décisions explicitement différées (non bloquantes)
- Composition exacte des presets "Minimum"/"Moyen" (étapes 2/5) — à valider avec l'utilisateur au moment du développement.
- Authentification future (locale/distante) — hors scope, écran d'accueil = simple point d'ancrage.
- Signature/notarization macOS et Windows (étape 10) — dépend du canal de distribution prévu, à trancher plus tard.

## Fichiers critiques
- `/home/nathan/tracker/src/lib.rs` (à créer, prérequis bloquant)
- `/home/nathan/tracker/src/consent.rs` (à créer, modèle central)
- `/home/nathan/tracker/tests/consent_parity.rs` (à créer, garde-fou anti-dérive)
- `/home/nathan/tracker/src-tauri/src/commands.rs` (à créer, point d'intégration IPC)
- `/home/nathan/tracker/src/report.rs`, `/home/nathan/tracker/src/markdown.rs`, `/home/nathan/tracker/src/xml.rs` (filtrage "np", étape 9 — le chantier le plus volumineux)
- `/home/nathan/tracker/Cargo.toml` (déclaration du workspace)
