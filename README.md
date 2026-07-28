# Tracker

Outil qui collecte des informations matérielles et logicielles sur la machine locale (Linux, macOS, Windows) et les exporte en JSON/Markdown/XML. Le projet a deux faces :

- **CLI** (`tracker`, racine du repo) : collecte systématique et export non filtré, sans interaction.
- **Client GUI** (`tracker-gui`, Tauri, dossier `src-tauri/` + `frontend/`) : même moteur de collecte, mais avec un écran de consentement (opt-in par catégorie ou par preset) avant l'export.

## Prérequis

- Rust (édition 2024) et Cargo
- Pour le client GUI uniquement : Node.js/npm (voir `docs/BUILD.md` pour les dépendances système par OS)

## Build & utilisation — CLI

```bash
cargo build --release
cargo run --release
```

Génère trois fichiers dans le répertoire courant (`tracker_report.json`, `.md`, `.xml`). Aucun argument de ligne de commande : toutes les catégories sont collectées et les trois formats sont toujours générés, sans filtrage.

## Build & utilisation — Client GUI

```bash
cd frontend && npm install
cargo tauri dev   # ou: npx --prefix frontend tauri dev
```

Détails d'empaquetage multiplateforme : `docs/BUILD.md`.

## Données collectées

Matériel (CPU, RAM, disques, réseau, GPU, écrans, batterie, capteurs, Bluetooth, USB, imprimantes, etc.), logiciel (OS, processus, services, tâches planifiées, applications installées, Docker/Podman, machines virtuelles, etc.) et navigateurs (versions, extensions). La collecte dégrade toujours silencieusement vers une valeur absente plutôt que d'échouer ; la couverture varie selon l'OS.

- Détail exhaustif de ce qui est collecté : `donnees_collectees.md`
- Fonctionnement du backend (modules de collecte, consentement, filtrage à l'export) : `README_backend_data_harvest.md`
- Fonctionnement du client GUI (onglets, IPC, flux de consentement) : `README_frontend_client.md`
- Comment les données sont envoyées/exportées (fichiers, historique SQLite, envoi HTTP distant) : `README_data_how_they_are_send.md`

## Consulter l'historique local (SQLite)

Chaque collecte est historisée, non filtrée, dans une base SQLite locale :

- **Emplacement** : `tracker.db` dans le répertoire de données utilisateur standard
  (Linux : `~/.local/share/tracker/tracker.db`, macOS : `~/Library/Application Support/com.tracker.tracker/tracker.db`, Windows : `%APPDATA%\tracker\tracker\data\tracker.db`)
- **Tables** : `snapshots` (JSON brut de chaque relevé), `hardware_summary`, `software_summary` (liées via `snapshot_id`)

Avec la CLI `sqlite3` :

```bash
# Lister les snapshots avec date lisible (heure locale)
sqlite3 -header -column ~/.local/share/tracker/tracker.db \
  "SELECT id, machine_id, datetime(collected_at_unix,'unixepoch','localtime') AS collected_at
   FROM snapshots ORDER BY collected_at_unix DESC;"

# Voir le résumé matériel/logiciel joint à chaque snapshot
sqlite3 -header -column ~/.local/share/tracker/tracker.db \
  "SELECT s.id, datetime(s.collected_at_unix,'unixepoch','localtime') AS collected_at,
          h.cpu_architecture, h.ram_total_mb, sw.os_name, sw.host_name
   FROM snapshots s
   LEFT JOIN hardware_summary h ON h.snapshot_id = s.id
   LEFT JOIN software_summary sw ON sw.snapshot_id = s.id;"

# Voir le JSON complet d'un snapshot précis (id=1)
sqlite3 ~/.local/share/tracker/tracker.db "SELECT raw_json FROM snapshots WHERE id=1;"
```

Une interface graphique comme [DB Browser for SQLite](https://sqlitebrowser.org/) fonctionne aussi. Détails du schéma et de la logique d'écriture : `README_data_how_they_are_send.md` (section « Historique local SQLite ») et `src/storage/mod.rs`.
