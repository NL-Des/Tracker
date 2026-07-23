# Build & empaquetage

Ce document couvre la compilation et l'empaquetage du client Tauri (`tracker-gui`,
dans `src-tauri/`) sur les trois OS cibles. Le crate `tracker` (CLI, `cargo build`/`cargo run`
à la racine) n'a besoin d'aucune de ces dépendances système.

## Dépendances système par OS

### Linux
- WebKitGTK — nom du paquet variable selon la distribution :
  - Debian/Ubuntu : `libwebkit2gtk-4.1-dev`
  - Fedora : `webkit2gtk4.1-devel`
  - Arch : `webkit2gtk-4.1`
- Autres paquets requis par Tauri v2 : `libgtk-3-dev`, `libayatana-appindicator3-dev`
  (ou `libappindicator3-dev` selon distro), `librsvg2-dev`, `patchelf`, `build-essential`
  (Debian/Ubuntu ; noms équivalents ailleurs).
- Cibles de bundle (`src-tauri/tauri.conf.json`, `bundle.targets: "all"`) : `.deb`, AppImage.

### Windows
- WebView2 Runtime : généralement préinstallé sur Windows 10/11 à jour. Sinon, installer
  le "WebView2 Runtime" (Evergreen) depuis Microsoft avant de lancer le bundle installé.
- Cibles de bundle : `.msi` (WiX) et `.exe` (NSIS).

### macOS
- WKWebView : natif, aucune dépendance système supplémentaire.
- Cibles de bundle : `.app`, `.dmg`.
- Signature / notarization : nécessaire pour une distribution hors Mac App Store sans
  avertissement Gatekeeper. Canal de distribution encore à trancher avec le client
  (décision différée, cf. `plan_client.md`).

## Build local

```bash
# Backend Rust (CLI + logique partagée + binaire GUI)
cargo build --workspace
cargo test --workspace

# Frontend
cd frontend
npm install
cd ..

# Mode développement (ouvre la fenêtre Tauri, hot-reload Vite)
npx --prefix frontend tauri dev

# Bundle de production pour l'OS courant
npx --prefix frontend tauri build
```

Le CLI Tauri (fourni par `@tauri-apps/cli`, devDependency de `frontend/package.json`)
cherche un dossier `src-tauri` **dans les sous-dossiers du répertoire courant** — il faut
donc l'invoquer depuis la racine du dépôt (où `src-tauri/` est bien un sous-dossier), pas
depuis `frontend/` lui-même. `npx --prefix frontend tauri ...` exécute le binaire installé
dans `frontend/node_modules` tout en gardant la racine du dépôt comme répertoire courant.

## Points d'attention lors du packaging

- Les appels système du crate `tracker` (`winreg`/`wmi` sous Windows, les commandes
  shell-out de `src/command.rs`, ex. `smartctl`) doivent être re-testés depuis le binaire
  **empaqueté** : les permissions et le `PATH` diffèrent d'un run en dev, et macOS peut
  bloquer une app non signée via Gatekeeper.
- Mesurer la taille finale du bundle et tester l'installation sur une machine "propre"
  (sans toolchain de développement installé).

## Intégration continue

`.github/workflows/ci.yml` fait tourner, sur une matrice Linux/macOS/Windows :
`cargo build --workspace`, `cargo test --workspace`, puis `npx tauri build`
(après installation des dépendances système Linux listées ci-dessus sur le runner Ubuntu).
