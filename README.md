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
