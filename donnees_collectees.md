# Données du projet `tracker`

> Ce document liste les données actuellement collectées par le code existant. Pour les données non encore collectées (roadmap), voir `Donnees_futures_a_collecter.md`. Pour l'explication du fonctionnement du backend, voir `README_backend_data_harvest.md`.

## Données collectées

### Matériel (`src/hardware/`, agrégé dans `HardwareInfo`, 21 modules)
- **CPU** (`cpu.rs`) : architecture, cœurs (usage/fréquence/marque par cœur), usage global, vulnérabilités Spectre/Meltdown (Linux), gouverneur de fréquence (Linux), `ProcessorId` (Windows), version du microcode (Linux).
- **Mémoire** (`memory.rs`) : RAM/swap totale et utilisée, détail par barrette (fabricant, numéro de série, capacité, fréquence — Windows uniquement).
- **Disques** (`disks.rs`) : nom, type, système de fichiers, point de montage, amovible, taille, modèle et numéro de série (Linux/Windows, best-effort macOS), santé S.M.A.R.T. sommaire (NVMe). Disques physiques et montages virtuels séparés.
- **Réseau** (`network.rs`) : interfaces (octets reçus/transmis cumulés, adresse MAC, adresses IPv4/IPv6, vitesse de liaison, type de connexion filaire/wifi/virtuel, débit instantané mesuré en Mbps), passerelle par défaut, serveurs DNS (Linux).
- **Wi-Fi** (`wifi.rs`) : SSID, force du signal, interface, débit de liaison (Mbps) — connexion(s) active(s) uniquement.
- **Périphériques PCI** (`pci_devices.rs`) : nom, classe.
- **Capteurs/composants** (`components.rs`) : label, températures actuelle/max/critique.
- **Batterie** (`battery.rs`) : fabricant, modèle, numéro de série, état, technologie, charge, santé, température, cycles, temps restant.
- **Carte mère/BIOS** (`motherboard.rs`) : fabricant, modèle, numéro de série (best-effort), version, infos BIOS, UUID machine (souvent inaccessible sans admin), état Secure Boot (Linux), version TPM (Linux/Windows).
- **GPU** (`gpu.rs`) : nom, fabricant, VRAM (Mo), version driver.
- **Écrans** (`display_monitor.rs`) : nom, dimensions, position, échelle, fréquence, écran principal/intégré, identifiants EDID (fabricant/modèle/série — Linux/Windows, best-effort, association par index).
- **Lecteurs optiques/disquettes** (`optical_drives.rs`) : nom, fabricant, type.
- **Périphériques génériques** (`peripherals.rs`) : nom, type (clavier, enceintes, ...).
- **Souris / manettes / touchpads** (`input_devices.rs`) : nom, par catégorie.
- **Caméras** (`camera.rs`) : nom.
- **Périphériques USB** (`usb_devices.rs`) : nom, fabricant, classification de classe (best-effort, "None" si non déterminable).
- **Périphériques Bluetooth appairés** (`bluetooth_devices.rs`) : nom.
- **Imprimantes / scanners** (`printers.rs`) : nom, type.
- **Ventilateurs** (`fans.rs`) : nom, vitesse RPM (souvent absente sur laptop).
- **Disposition du stockage** (`storage_layout.rs`) : table de partitions, volumes LVM, tableaux RAID logiciels (Linux principalement).
- **Profil d'alimentation** (`power_profile.rs`) : profil actif (ex: "balanced"), mode de veille.

### Logiciel (`src/software/`, agrégé dans `SoftwareInfo`, 21 modules)
- **OS** (`os_info.rs`) : nom, version noyau/OS, nom d'hôte, uptime.
- **Processus** (`processes.rs`) : nombre total + liste complète (PID, nom, CPU %, mémoire), triée par usage CPU.
- **Comptes utilisateurs** (`users.rs`) : nom, UID, GID, groupes, indicateur admin/sudo (dérivé des groupes sur Linux/macOS, de `net localgroup administrators` sur Windows) — comptes système, pas les sessions connectées.
- **Variables d'environnement** (`env_vars.rs`) : clé/valeur, avec redaction automatique (TOKEN/SECRET/KEY/PASSWORD/PWD/CREDENTIAL/AUTH).
- **Applications installées** (`installed_apps.rs`) : nom, version, éditeur, source de détection.
- **Runtimes de développement** (`dev_runtimes.rs`) : nom/version pour une liste courte (Python, Node.js, Java, Rust, Go, Ruby, PHP, .NET) si présents dans le `PATH`.
- **Services/démons** (`services.rs`) : nom, statut — lecture seule.
- **Tâches planifiées** (`scheduled_tasks.rs`) : nom, planning — utilisateur courant uniquement.
- **Démarrage automatique** (`autostart.rs`) : nom, commande — portée utilisateur uniquement.
- **Gestionnaires de paquets** (`packages.rs`) : gestionnaire + décompte de paquets (dpkg/apt, rpm, snap, flatpak, cargo, npm, brew).
- **Connexions réseau** (`network_connections.rs`) : protocole, adresse locale, état — visibles par l'utilisateur courant.
- **Environnement de bureau** (`desktop_env.rs`) : desktop, type de session, locale, fuseau horaire.
- **Historique des mises à jour** (`update_history.rs`) : date, description — 20 entrées les plus récentes.
- **Modules noyau chargés** (`kernel_modules.rs`) : nom, taille.
- **Images Docker** (`docker.rs`) : dépôt, tag, ID image, taille, date de création.
- **Volumes Docker** (`docker.rs`) : nom, driver, point de montage.
- **Machines virtuelles** (`virtual_machines.rs`) : nom, hyperviseur (VirtualBox/libvirt), état, identifiant.
- **Images / volumes Podman** (`podman.rs`) : mêmes champs que Docker (Podman n'est pas couvert pour containerd, dont le socket nécessite généralement root).
- **Services en échec** (`services.rs::collect_failed`) : unités systemd en échec (Linux).
- **Polices installées** (`fonts.rs`) : décompte + liste des familles dédoublonnées.
- **Configuration proxy système** (`proxy_config.rs`) : proxy HTTP/HTTPS/exceptions et source de détection (env/gsettings/scutil/registre).
- **Clés SSH** (`ssh_keys.rs`) : métadonnées des clés publiques uniquement (fichier, type, empreinte) — jamais le contenu privé.
- **État de sécurité** (`security_status.rs`) : pare-feu actif (Windows/macOS, best-effort Linux), statut de chiffrement disque (LUKS/FileVault, `None` sur Windows sans élévation), produit antivirus (Windows uniquement, via WMI SecurityCenter2).

### Navigateurs (`src/browsers/`)
- Nom, version (obtenue en exécutant `--version`), chemin, navigateur par défaut (bool), extensions installées (id/nom/version, lues dans le profil par défaut — Chrome/Chromium/Brave/Edge/Opera/Vivaldi et Firefox ; `None` si le profil n'a pas pu être localisé).

### Métadonnées du rapport (`src/report.rs`)
- Horodatage de génération (Unix), version de l'outil, avertissements de collecte (ex. UUID inaccessible, aucun écran/GPU/navigateur détecté).

Tout ceci est sérialisé dans `tracker_report.json` à la racine du projet.
