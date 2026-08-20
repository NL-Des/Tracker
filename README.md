# Tracker

**Objectifs et motivations**

Tracker est un projet qui avait deux objectifs majeurs :

-Découvrir et comprendre les données que l'on peut obtenir d'un ordinateur sans droits administrateurs.

-Apprendre à concevoir un projet de bout en bout avec l'IA. Concevoir, analyser le code soumis, compartimenter pour réduire la dispersion de l'attention de l'IA, construire une méthodologie de travail claire et efficace. Mais sans agents, pour étudier le comportement de l'IA et comprendre les limites parfois floues de ces actions.

***Bilan :***

-J'ai pu observer qu'un très grands nombre d'informations logicielles et matérielles sont disponibles et accessibles avec une grande facilité. Cela amène beaucoup de questions sur la sécurité et la circulation des informations. Mais cela ouvre aussi des portes pour développer des idées d'améliorations de ces éléments découverts.
J'ai pu apprendre et explorer beaucoup d'éléments qui m'étaient il y a peu encore inconnus. C'est une riche expérience pour s'ouvrir l'esprit sur de multiples champs techniques matériels et logiciels.

-Au niveau de la méthodologie de travail, j'ai pu explorer en profondeur ma manière de travailler avec l'IA. Comment bien rédiger les questions, les ordres et demandes d'informations. Quand renouveler les discussions, comment lui transmettre les informations, pourquoi communiquer au travers de documents en markdown plutôt qu'au travers de channels de discussions,... C'est un vaste chantier qui met en évidence la nécessité d'une maîtrise de la langue et du sujet technique. Car ces deux éléments offrent un pont de communication clair et efficace, pour aborder des sujets techniques complexes ou encore conceptuels, avec recul et critique.
L'accumulation des lignes des codes, des demandes et des notions, oblige l'utilisateur à réfléchir si il désire un résultat fonctionnel. 

Il faut anticiper les actions de l'IA, compartimenter le travail et l'information, structurer le projet, ne pas hésiter à retourner en arrière, et vérifier régulièrement le travail par des tests et des analyses. Même si ces derniers éléments sont fais par l'IA. Car cela est vite révélateur d'erreurs, de conceptions par l'utilisateur (moi) ou de développement (l'IA).
L'IA m'a permis de réaliser ce projet rapidement à côté de mes projets de formations, alors que je débute en Rust. Cela offre de nombreuses opportunités, mais demande un profonds apprentissage pour éviter l'inévitable plafonds de verre si l'on se laisse porter par l'IA. Il est de la responsabilité de chacun d'entretenir régulièrement ces compétences.

Ce projet est disponible pour toute personne le désirant, pour un usage non commercial ou liés à des prestations payantes. Soyez averti qu'il est un projet expérimental, donc la sécurité n'est pas au point pour un usage autre que dans un cadre d'études d'informations non confidentielles. Si vous désirez tout de même l'utiliser dans un cadre plus sérieux, je suis disponible pour répondre à vos questions et vous aider. 

**Définition :**

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

Génère trois fichiers dans le répertoire courant (`tracker_report.json`, `.md`, `.xml`) : toutes les catégories sont collectées et les trois formats sont toujours générés, sans filtrage.

Arguments optionnels pour l'envoi distant (config partagée avec le client GUI, voir `README_data_how_they_are_send.md`) :

```bash
cargo run --release -- --remote-url http://mon-serveur/report --remote-token monjeton  # active et enregistre
cargo run --release -- --remote-disable                                                 # désactive
```

## Build & utilisation — Client GUI

```bash
cd frontend && npm install
cd ..
cd src-tauri
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
