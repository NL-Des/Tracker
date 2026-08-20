# Envoi & export des données — vue d'ensemble

Ce document explique **tout ce qui fait sortir les données** de la collecte en mémoire (`SystemReport`) : écriture de fichiers, historisation locale et envoi à un serveur distant. Pour la collecte elle-même, le modèle de consentement et le mécanisme de filtrage `"np"`, voir `README_backend_data_harvest.md`. Pour l'UI qui pilote ces envois, voir `README_frontend_client.md`.

## 1. Vue d'ensemble

Une seule commande IPC déclenche les trois mécanismes : `collect_and_export(formats, output_dir)` (`src-tauri/src/commands.rs:71-141`). À chaque appel :

```
SystemReport::collect()
        │
        ├─▶ 1. Historique SQLite local        (toujours, non filtré)      src/storage/mod.rs
        │
        ├─▶ 2. Fichiers JSON/Markdown/XML      (filtré "np" côté GUI)     src/report.rs, markdown.rs, xml.rs
        │
        └─▶ 3. Envoi HTTP vers un serveur      (filtré "np", optionnel)   src/remote_export.rs
             distant (si activé par l'utilisateur)
```

Les trois branches sont indépendantes et **best-effort** : l'échec de l'une n'empêche jamais les autres de s'exécuter (voir le détail de gestion d'erreur dans chaque section).

## 2. Exports fichiers (JSON / Markdown / XML)

- Génération : `SystemReport::save_json_filtered` / `save_markdown_filtered` / `save_xml_filtered` (`src/report.rs`), qui appliquent le filtrage `"np"` décrit dans `README_backend_data_harvest.md` §5 avant d'écrire sur disque.
- Déclenchement : boucle sur le paramètre `formats: Vec<String>` de `collect_and_export` (`commands.rs:95-115`), écriture dans `output_dir` sous les noms `tracker_report.json/.md/.xml`.
- **CLI vs GUI** : le CLI (`src/main.rs`) appelle les variantes non filtrées (`save_json`/`save_markdown`/`save_xml`) — comportement historique inchangé, toujours les 3 formats, aucun consentement à charger. Le GUI charge `ConsentConfig` courant et applique le filtrage.
- Gestion d'erreur : une erreur d'écriture fichier fait échouer `collect_and_export` entier (`result.map_err(|e| e.to_string())?`, `commands.rs:113`) — c'est la seule des trois branches qui peut faire échouer la commande.

## 3. Historique local SQLite

Module `src/storage/mod.rs`. Sert à conserver un historique des collectes sur la machine, indépendamment des exports fichiers.

- **Emplacement** : `tracker.db` dans le répertoire de données utilisateur standard (`directories::ProjectDirs::from("com","tracker","tracker").data_dir()`, `db_path()` l.20-29).
- **Schéma** (`migrate()`, l.44-75) :
  - `snapshots(id, machine_id, collected_at_unix, schema_version, raw_json)` — `raw_json` contient le rapport complet sérialisé.
  - `hardware_summary(snapshot_id, cpu_architecture, cpu_core_count, ram_total_mb, disk_total_gb)`
  - `software_summary(snapshot_id, os_name, os_version, host_name)`
  - Index `idx_snapshots_machine_collected(machine_id, collected_at_unix)`.
  - `SCHEMA_VERSION = 1` : uniquement `CREATE TABLE IF NOT EXISTS`, aucune migration incrémentale réelle pour l'instant.
- **Écriture** : `record_snapshot(report)` (l.129-132), appelée en tout premier dans `collect_and_export` (`commands.rs:83-85`), **avant** le chargement du consentement.
- **⚠️ Point de confidentialité important** : cette branche stocke **toujours le rapport complet, non filtré**, quel que soit le consentement de l'utilisateur (commentaire explicite `commands.rs:81-82`). Le filtrage `"np"` ne s'applique qu'aux exports fichiers et à l'envoi HTTP.
- **Lecture** : commandes IPC `list_snapshots()` / `get_snapshot(id)` (`commands.rs:143-153`) exposées côté Rust mais **non consommées par le frontend actuel** — pas encore de bouton/UI pour parcourir cet historique.
- **Sécurité** : toutes les requêtes utilisent des paramètres liés (`rusqlite::params!`) — pas de concaténation de chaînes, donc pas d'injection SQL possible.
- **Gestion d'erreur** : `Result<_, String>`, échec seulement loggué (`eprintln!`, `commands.rs:84`), jamais bloquant pour la suite de la commande.

## 4. Envoi HTTP vers un serveur distant

Module `src/remote_export.rs`. **Aucun serveur de réception définitif n'existe encore** — seul le client est implémenté.

- **Configuration** (`RemoteExportConfig { enabled, url, auth_token }`, l.13-19) : persistée en JSON dans `remote_export.json`, dans le **même répertoire de configuration** que `consent.json` (réutilise `consent::config_dir()`, l.26). Ce fichier est **partagé entre le CLI et le GUI** : le configurer depuis l'un ou l'autre a le même effet.
  - **GUI** : onglet Paramètres (`frontend/src/settings.js`) — toggle d'activation, champ URL, champ token.
  - **CLI** : arguments `--remote-url <URL>` (active l'envoi et fixe l'URL), `--remote-token <TOKEN>` (optionnel, avec `--remote-url`), `--remote-disable` (désactive sans perdre l'URL enregistrée). Voir `src/main.rs`.
- **Envoi** (`send_report(config, json_body)`) :
  - No-op immédiat si `enabled = false`.
  - `reqwest::blocking::Client` avec timeout de **10 secondes** par tentative.
  - **Retry** : jusqu'à 3 tentatives avec un court backoff (300 ms puis 600 ms), mais **uniquement** sur erreur réseau/timeout ou statut `5xx`. Un statut `4xx` (ex. `401` d'authentification invalide) est considéré définitif et n'est **jamais retenté**.
  - `POST` avec `Content-Type: application/json`, en-tête `Authorization: Bearer ...` ajouté seulement si `auth_token` est renseigné.
  - Erreur si toutes les tentatives échouent ou si le serveur répond un statut non-2xx définitif.
- **Déclenchement GUI** : en toute fin de `collect_and_export`, après l'écriture des fichiers (`commands.rs`). Charge la config (erreur de lecture volontairement avalée — une config illisible ne doit pas bloquer les fichiers déjà écrits), et si `enabled`, envoie **`report.to_json_pretty_filtered(&consent)`** — le même JSON filtré `"np"` que l'export fichier JSON, jamais la version complète.
- **Déclenchement CLI** : après l'écriture des fichiers dans `src/main.rs`. Charge la même config `remote_export.json` et, si `enabled`, envoie `report.to_json_pretty()` — le CLI n'ayant pas de notion de consentement, la version complète (cohérent avec le comportement non filtré des exports fichiers CLI).
- **Commandes IPC** (GUI) : `get_remote_export_config()` / `save_remote_export_config(config)` (`commands.rs`), calquées sur `get_consent`/`save_consent`.
- **Frontend** : onglet "Paramètres" (`frontend/src/settings.js`), toggle d'activation + champ URL + champ token, ajouté à `TABS` dans `app.js`.
- **Gestion d'erreur** : best-effort — un échec de l'envoi distant (config illisible, réseau indisponible, statut HTTP en erreur après retries) ne fait jamais échouer l'export fichiers déjà écrit, et reste loggué via `eprintln!`. Côté GUI, le résultat est en plus **remonté au frontend** : `collect_and_export` renvoie une structure `CollectAndExportResult { written, remote_export: Option<{ success, error }> }` (`commands.rs`), affichée dans l'écran d'accueil (`frontend/src/home.js`) juste après le message d'export fichiers. Côté CLI, le résultat est simplement affiché sur stdout/stderr.

## 5. Comparatif des trois mécanismes

| | Fichiers (JSON/MD/XML) | Historique SQLite | Envoi HTTP distant |
|---|---|---|---|
| Déclenchement | à chaque collecte, selon les formats demandés | systématique, à chaque collecte GUI | automatique si activé (config partagée CLI/GUI) |
| Filtrage par consentement | oui (GUI) / non (CLI) | **jamais** (toujours complet) | oui (GUI) / non (CLI) |
| Peut faire échouer la commande | oui | non (loggué) | non (loggué) |
| Configuration utilisateur | dossier de sortie (`output_dir`) | aucune (chemin fixe) | URL + activation (onglet Paramètres GUI, ou `--remote-url`/`--remote-token`/`--remote-disable` en CLI) |
| Stockage | fichiers sur disque | `tracker.db` (SQLite) | aucun stockage local, transmis au serveur distant |

## 6. Sécurité

- **SQL** : requêtes systématiquement paramétrées (`rusqlite::params!`) — pas d'injection possible.
- **Stockage en clair** : `consent.json`, `remote_export.json` et `tracker.db` sont tous stockés sans chiffrement sur le disque local — cohérent entre les trois, mais à garder en tête si des données sensibles (mode de consentement "Maximum") transitent par l'historique SQLite non filtré.
- **HTTP** : timeout de 10s par tentative pour éviter un blocage indéfini si le serveur est injoignable ; jusqu'à 3 tentatives avec backoff sur erreur réseau/5xx, **aucun retry sur 4xx** (évite par exemple de marteler un serveur avec un token invalide) ; pas de certificate pinning particulier (TLS géré par `rustls` via le feature `rustls-tls` de `reqwest`).
- **Auth** : `auth_token` existe côté modèle et est configurable depuis l'UI ou le CLI. Aucun vrai serveur de réception n'implémente encore d'authentification à ce jour.

## 7. Tests

- `tests/export_filtering.rs` : couverture exhaustive du filtrage `"np"` pour les exports fichiers (JSON/Markdown/XML), plus `remote_export_sends_the_same_filtered_json_as_file_exports` qui vérifie que le JSON réellement transmis à `send_report` respecte le même filtrage que les fichiers (via un serveur HTTP jetable in-process).
- Tests unitaires `src/storage/mod.rs` : round-trip insertion/listing, récupération JSON par id, tri par date, cas id introuvable.
- Tests unitaires `src/remote_export.rs` : round-trip config load/save, no-op quand désactivé, POST correctement envoyé et reçu (serveur jetable in-process), échec propre sur statut 5xx, **pas de retry sur statut 4xx** (`send_report_does_not_retry_on_client_error_status`), **retry réussi après échecs transitoires** (`send_report_retries_on_server_error_and_eventually_succeeds`).
- Validation manuelle de bout en bout : pointer `--remote-url`/l'onglet Paramètres vers un serveur HTTP local jetable et vérifier la réception.
