# Tracker

Outil en ligne de commande qui collecte des informations matérielles, logicielles et autres sur la machine locale (Linux, macOS, Windows) et les exporte dans un rapport.

## Prérequis

- Rust (édition 2024) et Cargo

## Build

```bash
cargo build --release
```

## Utilisation

```bash
cargo run --release
```

L'exécution génère trois fichiers dans le répertoire courant :

- `tracker_report.json`
- `tracker_report.md`
- `tracker_report.xml`

Aucun argument de ligne de commande n'est actuellement disponible : toutes les catégories de données sont collectées et les trois formats sont toujours générés.

## Données collectées

Matériel (CPU, RAM, disques, réseau, GPU, écrans, batterie, capteurs, Bluetooth, USB, imprimantes, etc.), logiciel (OS, processus, services, tâches planifiées, applications installées, Docker/Podman, machines virtuelles, etc.) et navigateurs (versions, extensions). Le détail exhaustif des champs collectés se trouve dans `donnees.md`.

Sur certains modules, la couverture varie selon l'OS (voir les commentaires dans le code de chaque module concerné) : la collecte dégrade toujours silencieusement vers une valeur absente plutôt que d'échouer.
