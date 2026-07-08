# Données du projet `tracker`

## Liste 1 — Données actuellement connectées (collectées par le code existant)

### Matériel (`src/hardware/`, agrégé dans `HardwareInfo`)
- **CPU** (`cpu.rs`, via `sysinfo`) : architecture, nombre de cœurs, usage global (%), et par cœur : index, usage (%), fréquence (MHz), marque.
- **Mémoire** (`memory.rs`) : RAM totale/utilisée (Mo), swap total/utilisé (Mo).
- **Disques** (`disks.rs`) : nom, type, système de fichiers, point de montage, amovible (bool), taille totale/utilisée (Go).
- **Interfaces réseau** (`network.rs`) : nom de l'interface, octets reçus/transmis (compteurs locaux de la carte réseau, pas de requêtes internet).
- **Capteurs/composants** (`components.rs`) : label, température actuelle/max/critique (°C).
- **Batterie** (`battery.rs`, via `starship-battery`) : fabricant, modèle, état, technologie, charge (%), santé (%), température (°C), nombre de cycles, temps restant charge/décharge (min).
- **Carte mère/BIOS** (`motherboard.rs` + OS-specific) : fabricant, modèle, version, fabricant/version/date du BIOS, UUID machine (nécessite souvent des privilèges admin).
- **GPU** (`gpu.rs` + OS-specific) : nom, fabricant.
- **Écrans** (`display_monitor.rs`, via `display-info`) : nom, largeur/hauteur, position x/y, facteur d'échelle, fréquence (Hz), écran principal (bool).

### Logiciel (`src/software/`, agrégé dans `SoftwareInfo`)
- **OS** (`os_info.rs`) : nom, version du noyau, version OS, nom d'hôte, temps de fonctionnement (uptime).
- **Processus** (`processes.rs`) : nombre total, et liste des processus consommant >5% CPU (PID, nom, usage CPU %, mémoire Mo).
- **Comptes utilisateurs** (`users.rs`) : nom, UID, GID, groupes.
- **Variables d'environnement** (`env_vars.rs`) : clé/valeur, avec redaction automatique si la clé contient TOKEN/SECRET/KEY/PASSWORD/PWD/CREDENTIAL/AUTH.
- **Applications installées** (`installed_apps.rs` + OS-specific) : nom, version, éditeur, source de détection.

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
- Santé disque S.M.A.R.T. (secteurs défectueux, durée de vie estimée, cycles d'écriture SSD).
- Ventilateurs (vitesse RPM, courbes de refroidissement).
- Historique/courbe de charge de la batterie (dégradation dans le temps, pas juste une valeur instantanée).
- Débit réseau instantané (Mbps) plutôt que juste les compteurs cumulés d'octets.
- Latence/qualité de connexion Wi-Fi (SSID, force du signal, bande passante).
- Adresses IP locale/publique, fournisseur d'accès.
- Périphériques USB/Bluetooth connectés.
- Firmware/microcode CPU, version TPM.

### Logiciel / OS
- Historique de démarrage (crashs, temps de boot).
- Services/démons actifs (pas seulement les processus haute consommation).
- Ports réseau ouverts / connexions actives.
- Tâches planifiées (cron, Task Scheduler).
- Historique des mises à jour système/patchs de sécurité installés.
- Logs système récents (erreurs noyau, journaux d'événements).
- Paquets/dépendances par gestionnaire de paquets (apt, brew, npm global, cargo installés).
- Fichiers de démarrage automatique (autostart/startup items).

### Données d'usage / comportement (nécessiterait suivi dans le temps)
- Temps d'utilisation par application (pas seulement instantané CPU/mémoire).
- Historique de connexion/déconnexion utilisateur.
- Fréquence de lancement des applications.

### Navigateurs
- Extensions installées par navigateur (nom, version, éditeur, permissions demandées) — le champ `extensions` existe déjà dans `BrowserInfo` (Liste 1) mais n'est jamais rempli (`None`) actuellement.

### Métadonnées / qualité de collecte
- Bilan structuré (et non une simple liste de messages texte) : pour chaque champ attendu, statut collecté/échoué + raison de l'échec (permissions insuffisantes, capteur absent, plateforme non supportée, etc.), plutôt que la liste actuelle de chaînes libres dans `collection_warnings`.

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
- **Adresse MAC des interfaces réseau** : Linux via `/sys/class/net/*/address` (lecture libre) ; Windows/macOS sans privilège particulier.
- **Vitesse ventilateurs (RPM)** : Linux via `/sys/class/hwmon/*/fan*_input` (lecture libre, pas root).
- **Services/démons actifs** : `systemctl list-units` (Linux), `Get-Service` (Windows), `launchctl list` (macOS) — lecture seule, pas d'admin.
- **Ports réseau ouverts / connexions actives** : `ss`/`netstat` en mode utilisateur listent déjà les connexions du propre utilisateur (liste complète tous utilisateurs peut nécessiter root selon l'OS).
- **Paquets installés (apt/brew/npm/cargo)** : toujours accessible sans élévation.
- **Historique des mises à jour** (logs `apt history`, Windows Update history) : généralement lisible sans admin.
- **IP locale et IP publique** (via une requête sortante) : aucune élévation requise.