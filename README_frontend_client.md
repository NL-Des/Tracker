# Frontend & client de consentement — `tracker-gui`

Ce document explique le fonctionnement du client graphique Tauri qui s'intercale entre la collecte de données et son export, pour donner à l'utilisateur un contrôle explicite (opt-in) sur ce qui est partagé. Pour le fonctionnement du backend Rust et de la collecte elle-même, voir `README_backend_data_harvest.md`.

## 1. Stack technique

- **Backend GUI** : `src-tauri/` (package `tracker-gui`, Tauri v2), dépend du crate `tracker` via `path = ".."`.
- **Frontend** : `frontend/` — Vite + **JavaScript vanilla**, sans framework. Choix assumé pour rester léger malgré l'ajout d'une toolchain Node.js/npm en plus de Rust.
- Communication frontend ↔ backend exclusivement via les commandes IPC Tauri (`@tauri-apps/api/core`, fonction `invoke`).

## 2. Commandes IPC (`src-tauri/src/commands.rs`)

| Commande | Rôle |
|---|---|
| `set_locale(locale)` | Synchronise la langue du backend Rust (utilisée pour `collection_status`) sur celle choisie côté frontend. |
| `get_consent()` | Charge `ConsentConfig` depuis le disque (`tracker::consent::load()`), ou la config par défaut (tout désactivé) si aucun fichier n'existe. |
| `save_consent(config)` | Horodate `accepted_at_unix` puis sauvegarde `ConsentConfig` sur disque. |
| `list_hardware_fields()` / `list_software_fields()` | Retourne dynamiquement les clés de `HardwareConsent`/`SoftwareConsent` (introspection JSON) — le frontend ne code jamais les noms de champs en dur, seulement leur regroupement/libellés. |
| `get_preset(name)` | Retourne la `ConsentConfig` correspondant à un des 4 presets (`none`/`minimum`/`medium`/`maximum`), calculée côté Rust (`ConsentPreset::to_config()`). |
| `collect_and_export(formats, output_dir)` | `async`, exécutée via `tauri::async_runtime::spawn_blocking` (car `SystemReport::collect()` contient un `sleep` bloquant). Charge le consentement courant et appelle les variantes filtrées (`save_json_filtered`, etc.) — c'est le seul point du projet où le filtrage `"np"` est réellement appliqué. |

## 3. Architecture du frontend (`frontend/src/`)

- **`main.js`** — bootstrap : synchronise la locale backend sur la locale frontend, charge `getConsent()` une seule fois au démarrage (pas derrière un bouton "Commencer", car les onglets Hardware/Software lisent l'état dès leur montage), puis appelle `renderApp(root)`.
- **`app.js`** — système d'onglets maison : chaque page (`home`, `presets`, `hardware`, `software`, `network`) est montée **une seule fois** au démarrage ; changer d'onglet bascule juste la visibilité (`hidden`) sans recréer le DOM, pour ne jamais perdre une modification non enregistrée en changeant d'onglet.
- **`state.js`** — pub/sub minimal (`getConsentState`/`setConsentState`/`subscribe`) : état de consentement partagé entre onglets, pour que l'application d'un preset dans l'onglet "Niveaux globaux" se répercute immédiatement dans les onglets Hardware/Software déjà montés.
- **`i18n.js`** — `t(key)` lit dans `locales/{fr,en}.json` avec repli sur l'anglais puis sur la clé brute ; `setLocale()` bascule le dictionnaire local et notifie le backend Rust via `set_locale`.
- **`api.js`** — wrapper fin autour de `invoke()` pour les 7 commandes IPC.

## 4. Les 5 onglets

### Accueil (`home.js`)
Sélecteur de langue (FR/EN) + bouton "Récolter les données" qui appelle `collect_and_export(["json", "markdown", "xml"], ".")` (écrit dans le répertoire courant du processus, comme le CLI — pas de sélecteur de dossier dans cette première version). Aucune logique d'authentification : l'écran d'accueil reste un simple point d'ancrage.

### Niveaux globaux (`presets.js`)
Liste les 4 presets. Au clic sur un preset : `getPreset(name)` → `saveConsent(config)` → `setConsentState(config)`, qui propage automatiquement le changement aux onglets Hardware/Software déjà montés (via `subscribe`).

### Hardware (`hardware.js`)
8 sous-onglets couvrant les 24 champs de `HardwareConsent` : `storage`, `network`, `sensors`, `power`, `system`, `display`, `bus`, `peripherals`. Au montage, vérifie que les champs reçus de `list_hardware_fields()` correspondent exactement à ceux codés en dur dans les groupes (`isConsistent`) — en cas de divergence (ex. nouveau module ajouté côté Rust sans mise à jour du frontend), affiche un message d'erreur plutôt qu'une UI incomplète silencieuse. Sauvegarde uniquement sur clic explicite (pas à chaque case cochée), en ne fusionnant que la tranche `hardware` sur l'état global courant (pour ne pas écraser `software`/`browsers` avec une copie locale périmée).

### Software (`software.js`)
Même architecture, 8 sous-onglets pour les 24 champs de `SoftwareConsent` : `system`, `processes_users`, `applications`, `services_tasks`, `packages`, `containers_vm`, `network`, `security`. Particularité : `browsers` (qui vit dans `ConsentConfig` directement, pas dans `SoftwareConsent`, car `SystemReport.browsers` est une liste séparée) est affiché comme case à part dans le groupe `applications`, en dehors de la vérification de parité des champs.

### Réseau (`network.js`)
Volontairement vide — aucun module de collecte réseau dédié n'existe côté Rust, donc pas de `NetworkConsent`. Simple entrée de navigation avec message "à venir".

## 5. Flux de consentement de bout en bout

```
Démarrage → get_consent() → état partagé (state.js)
                                   ↓
     Preset appliqué  ──────────→  ou édition manuelle (Hardware/Software)
                                   ↓
                            save_consent(config)   (persisté dans consent.json)
                                   ↓
        Bouton "Récolter les données" → collect_and_export()
                                   ↓
       consent::load() côté Rust → export JSON/MD/XML avec "np" sur les champs désactivés
```

## 6. Internationalisation (frontend)

`frontend/src/locales/fr.json` et `en.json` — clés couvrant les onglets, les 4 presets (titre + description), les 8 groupes et 24 champs de Hardware, les 8 groupes et 24 champs + `browsers` de Software, et l'onglet Réseau.

## 7. Packaging (`src-tauri/tauri.conf.json`, `docs/BUILD.md`, CI)

- `tauri.conf.json` : identifiant `com.tracker.gui`, fenêtre 900×700, bundle actif pour toutes les cibles (`bundle.targets: "all"`), icônes dans `src-tauri/icons/`.
- `docs/BUILD.md` : dépendances système par OS (WebKitGTK + paquets GTK sur Linux, WebView2 sur Windows — généralement préinstallé, WKWebView natif sur macOS), cibles de bundle par OS (deb/AppImage, msi/nsis, dmg/app). Signature/notarization macOS et Windows **non tranchée** (dépend du canal de distribution prévu).
- CI (`.github/workflows/ci.yml`) : matrice Linux/macOS/Windows, `cargo build --workspace`, `cargo test --workspace`, puis `npx --prefix frontend tauri build` (exécuté depuis la racine du dépôt, car le CLI Tauri cherche `src-tauri` en sous-dossier du répertoire courant).
