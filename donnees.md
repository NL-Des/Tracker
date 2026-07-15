# Données du projet `tracker`

> Ce document est une vue d'ensemble synthétique. Pour le détail champ par champ de chaque structure, voir `bilan.md`.

## Liste 1 — Données actuellement collectées (collectées par le code existant)

### Matériel (`src/hardware/`, agrégé dans `HardwareInfo`, 19 modules)
- **CPU** (`cpu.rs`) : architecture, cœurs (usage/fréquence/marque par cœur), usage global, vulnérabilités Spectre/Meltdown (Linux), gouverneur de fréquence (Linux).
- **Mémoire** (`memory.rs`) : RAM/swap totale et utilisée.
- **Disques** (`disks.rs`) : nom, type, système de fichiers, point de montage, amovible, taille, santé S.M.A.R.T. sommaire (NVMe). Disques physiques et montages virtuels séparés.
- **Réseau** (`network.rs`) : interfaces (octets reçus/transmis cumulés, adresse MAC, adresses IPv4/IPv6, vitesse de liaison, type de connexion filaire/wifi/virtuel), passerelle par défaut, serveurs DNS (Linux).
- **Wi-Fi** (`wifi.rs`) : SSID, force du signal, interface — connexion(s) active(s) uniquement.
- **Périphériques PCI** (`pci_devices.rs`) : nom, classe.
- **Capteurs/composants** (`components.rs`) : label, températures actuelle/max/critique.
- **Batterie** (`battery.rs`) : fabricant, modèle, état, technologie, charge, santé, température, cycles, temps restant.
- **Carte mère/BIOS** (`motherboard.rs`) : fabricant, modèle, version, infos BIOS, UUID machine (souvent inaccessible sans admin), état Secure Boot (Linux).
- **GPU** (`gpu.rs`) : nom, fabricant, VRAM (Mo), version driver.
- **Écrans** (`display_monitor.rs`) : nom, dimensions, position, échelle, fréquence, écran principal/intégré.
- **Lecteurs optiques/disquettes** (`optical_drives.rs`) : nom, fabricant, type.
- **Périphériques génériques** (`peripherals.rs`) : nom, type (clavier, enceintes, ...).
- **Souris / manettes / touchpads** (`input_devices.rs`) : nom, par catégorie.
- **Caméras** (`camera.rs`) : nom.
- **Périphériques USB** (`usb_devices.rs`) : nom, fabricant (sans classification fine).
- **Périphériques Bluetooth appairés** (`bluetooth_devices.rs`) : nom.
- **Imprimantes / scanners** (`printers.rs`) : nom, type.
- **Ventilateurs** (`fans.rs`) : nom, vitesse RPM (souvent absente sur laptop).
- **Disposition du stockage** (`storage_layout.rs`) : table de partitions, volumes LVM, tableaux RAID logiciels (Linux principalement).
- **Profil d'alimentation** (`power_profile.rs`) : profil actif (ex: "balanced"), mode de veille.

### Logiciel (`src/software/`, agrégé dans `SoftwareInfo`, 16 modules)
- **OS** (`os_info.rs`) : nom, version noyau/OS, nom d'hôte, uptime.
- **Processus** (`processes.rs`) : nombre total + liste complète (PID, nom, CPU %, mémoire), triée par usage CPU.
- **Comptes utilisateurs** (`users.rs`) : nom, UID, GID, groupes (comptes système, pas les sessions connectées).
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

### Navigateurs (`src/browsers/`)
- Nom, version (obtenue en exécutant `--version`), chemin, navigateur par défaut (bool), extensions (champ réservé, toujours `None` actuellement).

### Métadonnées du rapport (`src/report.rs`)
- Horodatage de génération (Unix), version de l'outil, avertissements de collecte (ex. UUID inaccessible, aucun écran/GPU/navigateur détecté).

Tout ceci est sérialisé dans `tracker_report.json` à la racine du projet.

---

## Liste 2 — Données supplémentaires potentiellement exploitables (non collectées actuellement)

### Matériel / système bas niveau
- Historique d'usage CPU/mémoire dans le temps (séries temporelles au lieu d'un instantané unique).
- Fréquence et latence RAM (timings), nombre de barrettes, emplacements.
- Santé disque S.M.A.R.T. détaillée (secteurs défectueux, durée de vie estimée, cycles d'écriture SSD) — seul le statut sommaire PASSED/FAILED sur NVMe est collecté ; SATA/ATA nécessite généralement root.
- Courbes de refroidissement des ventilateurs et marque/modèle (vitesse RPM déjà collectée ; le reste vit dans les tables SMBIOS type 27, nécessite root).
- Historique/courbe de charge de la batterie (dégradation dans le temps, pas juste une valeur instantanée).
- Débit réseau instantané (Mbps) plutôt que juste les compteurs cumulés d'octets.
- Qualité de connexion Wi-Fi détaillée (bande passante) — SSID/force du signal déjà collectés.
- Adresse IP publique (nécessiterait une requête sortante ; adresse IP locale/passerelle/DNS déjà collectés).
- Classification fine des périphériques USB (stockage/réseau/autre, via descripteurs d'interface).
- Firmware/microcode CPU, version TPM.
- **Vitesse de liaison réseau et type de connexion sur macOS** — implémentés sur Linux/Windows, pas de source lecture-libre identifiée sur macOS pour l'instant.
- **RAID logiciel / LVM sur macOS et Windows** — implémentés sur Linux uniquement.

### Logiciel / OS
- Historique de démarrage (crashs, temps de boot).
- Logs système récents (erreurs noyau, journaux d'événements) — nécessite généralement root pour les logs complets.
- **Containerd** (`ctr`/`nerdctl`) — volontairement non couvert, son socket nécessite généralement root (Docker et Podman sont couverts).

### Données d'usage / comportement (nécessiterait suivi dans le temps)
- Temps d'utilisation par application (pas seulement instantané CPU/mémoire).
- Historique de connexion/déconnexion utilisateur.
- Fréquence de lancement des applications.

> Volontairement exclu du périmètre pour rester non intrusif : historique du presse-papiers, liste des fichiers récemment ouverts, tout suivi fin de l'usage applicatif au-delà d'un instantané.

### Navigateurs
- Extensions installées par navigateur (nom, version, éditeur, permissions demandées) — le champ `extensions` existe déjà dans `BrowserInfo` mais n'est jamais rempli (`None`) actuellement.

### Métadonnées / qualité de collecte
- Bilan structuré (et non une simple liste de messages texte) : pour chaque champ attendu, statut collecté/échoué + raison de l'échec (permissions insuffisantes, capteur absent, plateforme non supportée, etc.), plutôt que la liste actuelle de chaînes libres dans `collection_warnings`.

### Données externes (nécessiteraient une connexion réseau, actuellement absente du projet)
- Météo locale (via une API météo).
- Cours de cryptomonnaies/bourse (si pertinent pour un futur usage financier).
- Vérification de version la plus récente disponible pour les applications installées (comparaison avec un registre en ligne).
- Géolocalisation approximative (IP → ville) pour enrichir le rapport.
- Vulnérabilités connues (CVE) pour les logiciels installés détectés.

### Sécurité / conformité
- État du pare-feu (activé/désactivé, règles).
- État de l'antivirus/EDR.
- Chiffrement de disque (BitLocker/FileVault/LUKS actif ou non).
- Comptes avec privilèges administrateur/sudo.

### Accessible sans privilèges root/admin (parmi les points ci-dessus)
- **Marque/modèle CPU** : déjà présent (`brand`), aucune élévation requise.
- **`ProcessorId` CPU (Windows)** : via WMI `Win32_Processor`, pas d'admin requis.
- **Modèle/numéro de série des disques** : Linux via `/sys/block/*/device/{model,serial}` (lecture libre) ; Windows via WMI `Win32_DiskDrive` (pas d'admin requis) ; macOS via `diskutil info` (non privilégié).
- **Numéro de série de la carte mère (Windows)** : WMI `Win32_BaseBoard.SerialNumber` sans admin. (Sur Linux, `/sys/class/dmi/id/board_serial` est souvent restreint root-only selon la distribution — à vérifier au cas par cas.)
- **Numéro de série RAM (Windows)** : WMI `Win32_PhysicalMemory.SerialNumber` sans admin. (Sur Linux, `dmidecode -t 17` nécessite root ; pas d'équivalent sysfs non privilégié fiable.)
- **Fabricant/modèle/numéro de série des écrans (EDID)** : Linux via `/sys/class/drm/*/edid` (lecture libre) ; Windows via WMI `WmiMonitorID` (namespace `root/wmi`, pas d'admin requis).
- **Numéro de série batterie** : déjà exposé par `starship-battery` (`serial_number()`), aucune élévation requise.
- **Adresse MAC des interfaces réseau** : Linux via `/sys/class/net/*/address` (lecture libre) ; Windows/macOS sans privilège particulier — implémenté.
- **Vitesse ventilateurs (RPM)** : Linux via `/sys/class/hwmon/*/fan*_input` (lecture libre, pas root) — déjà implémenté.
- **Services/démons actifs** : `systemctl list-units` (Linux), `Get-Service` (Windows), `launchctl list` (macOS) — lecture seule, pas d'admin — déjà implémenté.
- **Ports réseau ouverts / connexions actives** : `ss`/`netstat` en mode utilisateur listent déjà les connexions du propre utilisateur (liste complète tous utilisateurs peut nécessiter root selon l'OS) — déjà implémenté pour le propre utilisateur.
- **Paquets installés (apt/brew/npm/cargo)** : toujours accessible sans élévation — déjà implémenté.
- **Historique des mises à jour** (logs `apt history`, Windows Update history) : généralement lisible sans admin — déjà implémenté.
- **IP locale et IP publique** (via une requête sortante) : aucune élévation requise pour l'IP locale (déjà implémentée via passerelle/DNS) ; IP publique non implémentée (nécessite une requête réseau sortante).

### Nécessite systématiquement root/admin
- UUID machine (`/sys/class/dmi/id/product_uuid` sur Linux).
- `dmidecode` sur Linux (BIOS bas niveau, RAM détaillée si sysfs insuffisant).
- Historique boot/crash détaillé, logs noyau complets (`dmesg` complet).
- Liste de tous les ports ouverts par tous les utilisateurs (selon OS).
- État antivirus/EDR, chiffrement de disque, règles de pare-feu détaillées.
