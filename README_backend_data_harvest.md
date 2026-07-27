# Backend & récolte de données — `tracker`

Ce document explique le fonctionnement interne du crate Rust `tracker` : comment les données sont collectées, structurées, filtrées selon le consentement de l'utilisateur, puis exportées. Pour la liste exhaustive des champs collectés, voir `donnees_collectees.md`. Pour le client graphique qui consomme ce backend, voir `README_frontend_client.md`.

## 1. Architecture du workspace

Le projet est un workspace Cargo à deux membres (`Cargo.toml` racine, `members = [".", "src-tauri"]`) :

- **`tracker`** (racine) : à la fois une bibliothèque (`src/lib.rs`, package Cargo `[lib]`) et un binaire CLI (`src/main.rs`, `[[bin]]`). Toute la logique de collecte, de modélisation et d'export vit dans la lib ; le binaire CLI ne fait qu'appeler `SystemReport::collect()` puis exporter en JSON/Markdown/XML, sans filtrage.
- **`src-tauri`** (package `tracker-gui`) : dépend de `tracker` via `path = ".."` et l'utilise comme bibliothèque pour construire le client graphique (voir `README_frontend_client.md`).

`src/lib.rs` expose 9 modules publics : `browsers`, `command`, `consent`, `hardware`, `markdown`, `os_dispatch`, `report`, `software`, `xml`, et ré-exporte `SystemReport` (`pub use report::SystemReport`).

## 2. Modules de collecte

### Matériel (`src/hardware/`, 21 modules → `HardwareInfo`)
`cpu`, `memory`, `disks`, `network`, `wifi`, `pci_devices`, `components`, `battery`, `motherboard`, `gpu`, `display_monitor`, `optical_drives`, `peripherals`, `input_devices`, `camera`, `usb_devices`, `bluetooth_devices`, `printers`, `fans`, `storage_layout`, `power_profile`.

### Logiciel (`src/software/`, 21 modules → `SoftwareInfo`)
`os_info`, `processes`, `users`, `env_vars`, `installed_apps`, `dev_runtimes`, `services` (inclut les services en échec), `scheduled_tasks`, `autostart`, `packages`, `network_connections`, `desktop_env`, `update_history`, `kernel_modules`, `docker` (images + volumes), `virtual_machines`, `podman` (images + volumes, réutilise les structs de `docker`), `fonts`, `proxy_config`, `ssh_keys`, `security_status`.

### Navigateurs (`src/browsers/`)
Détection Chrome/Chromium/Brave/Edge/Opera/Vivaldi et Firefox par OS (`linux.rs`/`macos.rs`/`windows.rs`), avec lecture des extensions installées (`extensions.rs`, profils Chromium et Firefox).

Chaque module suit la même philosophie : **collecte best-effort, sans droits root/admin**, et **infaillible par design** — une donnée indisponible produit `None`/un `Vec` vide plutôt qu'une erreur qui interromprait la collecte. Le détail champ par champ de chaque module (avec les commandes/fichiers systèmes utilisés par OS) est dans `donnees_collectees.md`.

## 3. Le rapport (`src/report.rs`)

`SystemReport` agrège `hardware: HardwareInfo`, `software: SoftwareInfo`, `browsers: Vec<BrowserInfo>`, plus des métadonnées (`generated_at_unix`, `tool_version`, `collection_status: Vec<FieldCollectionStatus>`).

`SystemReport::collect()` :
1. Initialise `sysinfo::System`, avec un double rafraîchissement CPU (`sysinfo::MINIMUM_CPU_UPDATE_INTERVAL` de délai) car `sysinfo` a besoin de deux mesures espacées pour calculer un usage CPU fiable — ce `sleep` bloquant est la raison pour laquelle la commande IPC `collect_and_export` côté GUI l'exécute dans `spawn_blocking` (voir `README_frontend_client.md`).
2. Appelle `hardware::collect()`, `software::collect()`, `browsers::collect()`.
3. Génère un bilan structuré (`collection_status`, liste de `FieldCollectionStatus { field, status, reason }`) pour les champs jugés fragiles : UUID machine, écrans, GPU, navigateurs — statut `"collected"`/`"unavailable"` avec raison traduite le cas échéant.

## 4. Modèle de consentement (`src/consent.rs`)

- `HardwareConsent` (24 booléens) et `SoftwareConsent` (24 booléens) : un booléen par champ de `HardwareInfo`/`SoftwareInfo` — la granularité de consentement est **par module entier**, pas par champ individuel à l'intérieur d'un module (ex. une case "CPU", pas une case par sous-champ de `CpuInfo`). Les noms de champs doivent rester strictement identiques à ceux de `HardwareInfo`/`SoftwareInfo` ; ceci est vérifié automatiquement par `tests/consent_parity.rs`, qui compare les clés sérialisées de `ConsentConfig::default()` aux constantes `HARDWARE_FIELDS`/`SOFTWARE_FIELDS` (définies dans `hardware/mod.rs`/`software/mod.rs`) et échoue en cas de dérive (ex. nouveau module de collecte ajouté sans case de consentement correspondante).
- `ConsentConfig { version, accepted_at_unix, hardware, software, browsers }` : sérialisable JSON, persisté sur disque via `directories::ProjectDirs::from("com", "tracker", "tracker")` (répertoire de configuration standard par OS), fichier `consent.json`. `accepted_at_unix` est horodaté à chaque sauvegarde, pour tracer quand le consentement a été donné.
- `ConsentPreset` : 4 niveaux prédéfinis (`None`, `Minimum`, `Medium`, `Maximum`) avec une composition tranchée en `consent.rs` (lignes 162-273) :
  - **None** : tout désactivé.
  - **Minimum** : matériel technique non identifiant uniquement (CPU, mémoire, disques, composants, batterie, carte mère, GPU, PCI, écrans, lecteurs optiques, ventilateurs, stockage, alimentation) ; côté logiciel, seul `os` ; aucun réseau, périphérique, navigateur.
  - **Medium** : tout le matériel + logiciel "environnement sans données personnelles" (apps installées, runtimes dev, services, tâches planifiées, paquets, conteneurs/VM, polices, proxy, sécurité) ; exclut processus, comptes utilisateurs, variables d'environnement, connexions réseau, clés SSH, navigateurs.
  - **Maximum** : tout activé, y compris les champs les plus identifiants (utilisateurs, clés SSH, navigateurs).

## 5. Filtrage à l'export — l'indicateur `"np"`

Décision d'architecture : la collecte interne (`SystemReport::collect()`) reste **toujours exhaustive**, indépendamment du consentement — le filtrage n'intervient qu'**à l'export**, pour permettre l'audit (traçabilité de ce qui a été collecté vs partagé). Chaque champ désactivé est remplacé par la chaîne `"np"` plutôt qu'omis.

- **JSON** : `SystemReport::to_json_pretty_filtered(&self, consent)` sérialise le rapport en `serde_json::Value`, puis `filter_module()` introspecte dynamiquement `serde_json::to_value(&consent.hardware/software)` pour remplacer chaque clé désactivée par `"np"` — pas de ~48 `if` codés en dur, la fonction reste générique quel que soit le nombre de champs.
- **Markdown/XML** (`src/markdown.rs`, `src/xml.rs`) : `generate(report, consent)` vérifie le flag correspondant avant d'écrire chaque bloc, sinon écrit un placeholder `"np"`.
- **CLI vs GUI** : `main.rs` (CLI) appelle `save_json`/`save_markdown`/`save_xml` — comportement historique inchangé, jamais filtré (en interne, ces fonctions utilisent `ConsentPreset::Maximum.to_config()`, donc rien n'est masqué). Le GUI (`src-tauri`) appelle les variantes `*_filtered`, qui chargent le `ConsentConfig` courant de l'utilisateur avant l'export.
- Couverture de test : `tests/export_filtering.rs` boucle sur chacun des champs `HARDWARE_FIELDS`/`SOFTWARE_FIELDS`, désactive un champ à la fois, et vérifie le `"np"` en JSON/Markdown/XML, plus le cas `browsers` et le cas "tout activé = rien filtré".

## 6. Internationalisation (backend)

`rust-i18n` (`rust_i18n::i18n!("locales", fallback = "en")` dans `lib.rs`), fichiers `locales/fr.yml`/`locales/en.yml`. Le scope est **volontairement limité** aux `reason` de `collection_status` dans `report.rs` (les titres de section codés en dur dans `markdown.rs`/`xml.rs` restent en français, hors périmètre v1). La locale est positionnée via `rust_i18n::set_locale(...)`, appelée par le CLI (`main.rs`, via `LANG`) ou par la commande IPC `set_locale` côté GUI.

## 7. Tests

- `tests/consent_parity.rs` — garde-fou anti-dérive entre `HardwareConsent`/`SoftwareConsent` et `HARDWARE_FIELDS`/`SOFTWARE_FIELDS`.
- `tests/export_filtering.rs` — couverture exhaustive du filtrage `"np"` par champ et par format.
- Tests unitaires dans `consent.rs` (presets, sérialisation, round-trip save/load) et `report.rs` (fallback i18n).

## 8. Dépendances principales

`sysinfo`, `starship-battery`, `display-info` (collecte cross-plateforme), `winreg`/`wmi` (Windows uniquement), `serde`/`serde_json` (modèle + export), `directories` (persistance du consentement), `rust-i18n` (traduction des avertissements).
