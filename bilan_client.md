# Bilan préparatoire — Client GUI de confidentialité et télémétrie opt-in

## 1. Contexte et objectif

Le crate `tracker` (Rust, édition 2024) est aujourd'hui un outil **CLI** fonctionnel : il collecte, en une seule exécution, l'ensemble des données matérielles et logicielles de la machine locale et les exporte systématiquement dans trois formats (`tracker_report.json`, `tracker_report.md`, `tracker_report.xml`), via `src/report.rs`, `src/markdown.rs` et `src/xml.rs`. L'inventaire exhaustif de ce qui est collecté — module par module, champ par champ — est déjà documenté dans `bilan_data.md` (détail par structure) et `donnees_collectees.md` (vue de synthèse), et ne sera pas dupliqué ici.

L'objectif de la présente étape est de préparer la conception d'un **client GUI multiplateforme** (Linux/macOS/Windows) qui vienne s'intercaler avant la collecte/l'export, pour donner à l'utilisateur un contrôle explicite (consentement / opt-in) sur ce qui est partagé, avec :
- une page d'accueil minimale (bouton d'entrée, point d'ancrage pour une future authentification) ;
- un panneau à onglets : niveaux globaux prédéfinis, Hardware (sous-onglets par composant), Software (sous-onglets par catégorie), Réseau (réservé, vide pour l'instant) ;
- une internationalisation pilotée par fichiers JSON.

Ce document ne contient pas de code : c'est un bilan d'analyse destiné à cadrer l'architecture avant l'implémentation.

## 2. Choix d'architecture actés

Les quatre décisions suivantes ont été arbitrées et servent de base au reste de l'analyse :

### 2.1 Framework GUI : Tauri
Le frontend sera écrit en HTML/CSS/JS (ou un petit framework JS léger type Svelte/vanilla), avec un backend Rust exposé via des commandes IPC (`#[tauri::command]`). Implications :
- Ajoute une **toolchain Node.js/npm** au projet, en plus de Rust — à mettre en balance avec l'exigence de simplicité/légèreté du cahier des charges ; c'est un compromis assumé, pas un point à reconsidérer ici.
- Structure de projet typique : un dossier `src-tauri/` (backend Rust, `tauri.conf.json`, commandes IPC) + un dossier frontend (bundlé, Vite recommandé pour rester léger).
- L'i18n est prise en charge côté Rust via la crate `rust-i18n` (macro + fichiers de locale) ; son intégration avec le frontend Tauri/JS reste à préciser à l'implémentation.

### 2.2 Architecture projet : même workspace Cargo
Le GUI ne sera pas un dépôt séparé lisant des fichiers exportés : `src-tauri` consommera directement `tracker` comme bibliothèque, dans le même workspace Cargo.

**Prérequis concret identifié dans le code actuel** : `src/main.rs` déclare aujourd'hui les modules directement (`mod hardware; mod software; mod browsers; ...`), et il n'existe pas de `src/lib.rs`. Le crate n'est donc **pas consommable comme bibliothèque en l'état** — une extraction en `src/lib.rs` exposant `report`, `hardware`, `software`, `browsers` est un prérequis de refactoring avant tout code GUI, indépendant du reste du travail de conception.

Cible : workspace avec au moins trois membres — la lib `tracker`, le binaire CLI existant (conservé tel quel), et `src-tauri` (dépendance sur la lib via chemin relatif).

### 2.3 Granularité du consentement : par module
Le contrôle granulaire par case à cocher portera sur des **modules entiers** (ex. « CPU », « Batterie », « Processus »), pas sur des champs individuels. Une checkbox = un bloc de données complet, aligné sur les champs de `HardwareInfo`/`SoftwareInfo`. Ce choix simplifie fortement la modélisation et l'UI par rapport à un consentement au champ (qui aurait imposé des dizaines de cases par sous-onglet).

### 2.4 Filtrage à l'export : décision actée
La collecte reste exhaustive en interne (`SystemReport::collect()` inchangé, aucun filtrage à la source). Le filtrage se fait **à l'export** : chaque section désactivée par l'utilisateur est marquée d'un indicateur « np » dans le JSON/MD/XML généré, plutôt que d'être purement et simplement omise. Cela permet l'audit (traçabilité de ce qui a été collecté vs partagé), au prix de conserver en mémoire/disque, le temps de l'export, des données non consenties.

## 3. Verrous techniques et points de blocage

- **i18n via `rust-i18n`** : fichiers de locale dans `locales/`, chargés via la macro `t!()`. Langues cibles pour la v1 : FR + EN uniquement. S'appuyer sur le mécanisme de fallback de la crate si une clé manque dans une langue, et internationaliser aussi les messages actuellement générés côté Rust (ex. `collection_warnings` dans `report.rs`, aujourd'hui des chaînes françaises en dur) avec la même crate, pas seulement les libellés du frontend.
- **Synchronisation consentement ↔ modèle de données** : le `ConsentConfig` (une clé booléenne par module) devra rester aligné avec les structs `HardwareInfo`/`SoftwareInfo`, qui comptent une quarantaine de modules au total. Risque de dérive si un nouveau module de collecte est ajouté sans case à cocher correspondante — un test ou une convention de nommage stricte peut aider à limiter ce risque.
- **Persistance des préférences** : choisir où stocker le consentement (répertoire de configuration standard par OS, via une crate comme `dirs`/`directories`), le format (JSON), et surtout prévoir un **horodatage/versionnement du consentement** pour disposer d'une traçabilité de ce qui a été accepté, et quand.
- **Empaquetage multiplateforme Tauri** : dépendances système de la webview par OS (WebKitGTK sur Linux, WebView2 sur Windows — généralement déjà présent mais à vérifier sur les cibles de test, WKWebView sur macOS), question de la signature/notarization macOS et Windows en cas de distribution hors store, et taille finale du bundle.
- **Écran d'accueil et future authentification** : concevoir le bouton d'entrée comme un point d'ancrage simple (routage frontend) sans sur-ingénierie immédiate — aucune logique d'authentification à implémenter maintenant.
- **Onglet Réseau vide** : se limiter à l'entrée de navigation, sans contenu de remplissage inutile en v1.

## 4. Questions ouvertes pour le client

- Le futur module d'authentification prévu sur l'écran d'accueil sera-t-il local (simple profil utilisateur) ou distant (compte serveur) ? Cela conditionne le routage frontend à prévoir dès maintenant.
- Y a-t-il une contrainte de délai ou de jalon académique à respecter pour caler le planning de la section 5 ?

## 5. Plan d'action étape par étape

1. **Restructuration workspace** : extraire `src/lib.rs` depuis les `mod` de `main.rs`, conserver le binaire CLI existant tel quel, ajouter `src-tauri` comme second membre du workspace.
2. **Modèle de consentement** : définir `ConsentConfig` (un booléen par module, avec les 4 presets Aucun/Minimum/Moyen/Maximum comme mappings prédéfinis), sérialisation JSON, chargement/sauvegarde dans le répertoire de configuration utilisateur.
3. **Squelette Tauri** : initialisation du projet, commandes IPC de base (`get_consent`, `save_consent`, `collect_and_export`), écran d'accueil avec bouton d'entrée et routage frontend minimal.
4. **Plomberie i18n** : structure des fichiers de locale `rust-i18n` (`locales/`), intégration avec le frontend Tauri/JS, sélecteur de langue, internationalisation des messages Rust existants (`collection_warnings`).
5. **Onglet 1 — Niveaux globaux** : interface + mapping des 4 presets vers `ConsentConfig`.
6. **Onglet 2 — Hardware** : sous-onglets par composant, en reprenant la liste des modules déjà inventoriée dans `bilan_data.md`/`donnees_collectees.md`, une case à cocher par module.
7. **Onglet 3 — Software** : même principe pour les modules logiciels.
8. **Onglet 4 — Réseau** : entrée de navigation vide, réservée pour une itération future.
9. **Implémentation du filtrage à l'export** (indicateur « np » par section désactivée, cf. 2.4).
10. **Tests multiplateformes** (Linux/macOS/Windows) et empaquetage Tauri (bundler, taille, dépendances webview par OS).
