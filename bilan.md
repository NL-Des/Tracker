# Bilan du projet `tracker`

## Liste 1 — Données collectées (détail champ par champ, d'après le code source)

### Matériel

#### CPU (`src/hardware/cpu.rs` → `CpuInfo`)
- `architecture: String` — architecture du processeur (ex: x86_64)
- `core_count: usize` — nombre de cœurs logiques
- `global_usage_percent: f32` — usage CPU global (%)
- `cores: Vec<CoreInfo>` — par cœur :
  - `index: usize`
  - `usage_percent: f32`
  - `frequency_mhz: u64`
  - `brand: String`
- `vulnerabilities: Vec<VulnerabilityInfo>` — statuts des mitigations Spectre/Meltdown/etc. (`name`, `status`), lecture libre de `/sys/devices/system/cpu/vulnerabilities/*` sur Linux ; vide sur les autres OS
- `scaling_governor: Option<String>` — gouverneur de fréquence actif (performance/powersave/...), Linux uniquement via sysfs

#### Mémoire (`src/hardware/memory.rs` → `MemoryInfo`)
- `total_mb: u64` — RAM totale
- `used_mb: u64` — RAM utilisée
- `total_swap_mb: u64` — swap total
- `used_swap_mb: u64` — swap utilisé

#### Disques (`src/hardware/disks.rs` → `(Vec<DiskInfo>, Vec<DiskInfo>)`)
- `name: String`
- `kind: String` — type (SSD, HDD, etc., ou `Unknown` pour les montages virtuels)
- `file_system: String`
- `mount_point: String`
- `is_removable: bool`
- `total_gb: u64`
- `used_gb: u64`
- `smart_health: Option<String>` — santé S.M.A.R.T. sommaire ("PASSED"/"FAILED") via `smartctl -H`, tentée uniquement sur les disques NVMe (souvent lisible sans root, contrairement au SATA/ATA) ; `None` si `smartctl` absent ou échec

`collect()` retourne un tuple (disques physiques, montages virtuels), séparés selon que `kind == "Unknown"` (overlay Docker/containerd, etc.). Exposés dans `HardwareInfo` sous deux champs distincts : `disks` et `virtual_disks`.

#### Réseau (`src/hardware/network.rs` → `NetworkInfo`)
- `interfaces: Vec<NetworkInterfaceInfo>` :
  - `interface_name: String`
  - `received_bytes: u64` — cumulé depuis le démarrage
  - `transmitted_bytes: u64` — cumulé depuis le démarrage
- `default_gateway: Option<String>` — passerelle par défaut, via `ip route show default` (Linux, lecture libre)
- `dns_servers: Vec<String>` — serveurs DNS configurés, via `/etc/resolv.conf` (Linux, lecture libre)

#### Wi-Fi (`src/hardware/wifi.rs` → `Vec<WifiNetworkInfo>`, OS-specific)
- `ssid: String`
- `signal_percent: Option<i32>`
- `interface: Option<String>`

Connexion(s) Wi-Fi active(s) uniquement, via `nmcli` (Linux), `airport -I` (macOS), `netsh wlan show interfaces` (Windows) — tous en lecture libre. Vide si aucune interface Wi-Fi active (ex : connecté en filaire).

#### Périphériques PCI (`src/hardware/pci_devices.rs` → `Vec<PciDeviceInfo>`, OS-specific)
- `name: String`
- `class: String`

Via `lspci` (Linux), `system_profiler SPPCIDataType` (macOS), WMI `Win32_PnPEntity` filtré sur `PCI\` (Windows) — tous en lecture libre.

#### Capteurs/composants (`src/hardware/components.rs` → `Vec<ComponentInfo>`)
- `label: String`
- `temperature_celsius: Option<f32>`
- `max_temperature_celsius: Option<f32>`
- `critical_temperature_celsius: Option<f32>`

#### Batterie (`src/hardware/battery.rs` → `Vec<BatteryInfo>`)
- `vendor: Option<String>`
- `model: Option<String>`
- `state: String`
- `technology: String`
- `state_of_charge_percent: f32`
- `state_of_health_percent: f32`
- `temperature_celsius: Option<f32>`
- `cycle_count: Option<u32>`
- `time_to_full_minutes: Option<f32>`
- `time_to_empty_minutes: Option<f32>`

#### Carte mère/BIOS (`src/hardware/motherboard.rs` → `MotherboardInfo`, OS-specific)
- `vendor: Option<String>`
- `model: Option<String>`
- `version: Option<String>`
- `bios_vendor: Option<String>`
- `bios_version: Option<String>`
- `bios_date: Option<String>`
- `machine_uuid: Option<String>` — souvent `None` sans privilèges admin
- `secure_boot: Option<String>` — état Secure Boot ("enabled"/"disabled"), via `mokutil --sb-state` (Linux, lecture libre sur la plupart des distributions)

#### GPU (`src/hardware/gpu.rs` → `Vec<GpuInfo>`, OS-specific)
- `name: String`
- `vendor: Option<String>`

#### Écrans (`src/hardware/display_monitor.rs` → `Vec<MonitorInfo>`, via `display-info`)
- `name: String`
- `width: u32`
- `height: u32`
- `x: i32`
- `y: i32`
- `scale_factor: f32`
- `frequency_hz: f32`
- `is_primary: bool`
- `is_builtin: bool` — écran intégré (laptop) ou externe

#### Lecteurs optiques/disquettes (`src/hardware/optical_drives.rs` → `Vec<OpticalDriveInfo>`, OS-specific)
- `name: String`
- `vendor: Option<String>`
- `kind: String`

#### Périphériques génériques (`src/hardware/peripherals.rs` → `Vec<PeripheralInfo>`, OS-specific)
- `name: String`
- `kind: String` — ex : "Clavier", "Enceintes"

#### Souris / Manettes / Touchpads (`src/hardware/input_devices.rs` → `InputDevices`, OS-specific)
- `mice: Vec<InputDeviceInfo>`
- `gamepads: Vec<InputDeviceInfo>`
- `touchpads: Vec<InputDeviceInfo>`
- Chaque `InputDeviceInfo` : `name: String`

#### Caméras (`src/hardware/camera.rs` → `Vec<CameraInfo>`, OS-specific)
- `name: String`

#### Périphériques USB (`src/hardware/usb_devices.rs` → `Vec<UsbDeviceInfo>`, OS-specific)
- `name: String`
- `vendor: Option<String>`

Note : pas de classification fine (stockage/réseau/autre), jugé hors scope pour un inventaire.

#### Périphériques Bluetooth appairés (`src/hardware/bluetooth_devices.rs` → `Vec<BluetoothDeviceInfo>`, OS-specific)
- `name: String`

#### Imprimantes / Scanners (`src/hardware/printers.rs` → `Vec<PrinterInfo>`, OS-specific)
- `name: String`
- `kind: String` — "Imprimante" ou "Scanner"

#### Ventilateurs (`src/hardware/fans.rs` → `Vec<FanInfo>`, OS-specific)
- `name: String`
- `speed_rpm: Option<u32>` — souvent absent sur laptops. Pas de champ marque/modèle : cette info vit dans les tables SMBIOS (type 27), inaccessible sans droits root.

Toutes ces catégories sont infaillibles par design : absence de périphérique détectable ou erreur d'accès matériel renvoient simplement un `Vec` vide (ou des `Vec` vides pour `InputDevices`).

### Logiciel

#### OS (`src/software/os_info.rs` → `OsInfo`)
- `name: Option<String>`
- `kernel_version: Option<String>`
- `os_version: Option<String>`
- `host_name: Option<String>`
- `uptime_seconds: u64`

#### Processus (`src/software/processes.rs` → `ProcessSummary`)
- `total_count: usize`
- `processes: Vec<ProcessInfo>` — liste **complète** de tous les processus, triée par usage CPU décroissant (plus de filtre à >5%) :
  - `pid: u32`
  - `name: String`
  - `cpu_usage_percent: f32`
  - `memory_mb: u64`

#### Comptes utilisateurs (`src/software/users.rs` → `Vec<UserAccountInfo>`)
- `name: String`
- `uid: String`
- `gid: String`
- `groups: Vec<String>`

Note : liste les comptes système (type `/etc/passwd`), pas les sessions actuellement connectées.

#### Variables d'environnement (`src/software/env_vars.rs` → `Vec<EnvVarInfo>`)
- `key: String`
- `value: String` — remplacé par `***REDACTED***` si la clé contient TOKEN, SECRET, KEY, PASSWORD, PWD, CREDENTIAL ou AUTH

#### Applications installées (`src/software/installed_apps.rs` → `Vec<InstalledAppInfo>`, OS-specific)
- `name: String`
- `version: Option<String>`
- `publisher: Option<String>`
- `source: String` — méthode de détection (ex: "desktop-file", "registry", "app-bundle")

#### Runtimes de développement (`src/software/dev_runtimes.rs` → `Vec<DevRuntimeInfo>`)
- `name: String`, `version: String`
- Détection par exécution de `<binaire> --version` pour une liste courte non exhaustive (Python, Node.js, Java, Rust, Go, Ruby, PHP, .NET). Absent de la liste si le binaire n'est pas dans le `PATH`.

#### Services / démons (`src/software/services.rs` → `Vec<ServiceInfo>`, OS-specific)
- `name: String`, `status: String`
- Via `systemctl list-units --type=service` (Linux), `launchctl list` (macOS), WMI `Win32_Service` (Windows) — tous en lecture seule, aucune élévation requise.

#### Tâches planifiées (`src/software/scheduled_tasks.rs` → `Vec<ScheduledTaskInfo>`, OS-specific)
- `name: String` (commande), `schedule: String`
- Uniquement celles de l'utilisateur courant : `crontab -l` (Linux/macOS), WMI `MSFT_ScheduledTask` (Windows). Ne couvre pas les tâches d'autres comptes/système (nécessiterait root/admin).

#### Démarrage automatique (`src/software/autostart.rs` → `Vec<AutostartEntryInfo>`, OS-specific)
- `name: String`, `command: Option<String>`
- Portée utilisateur uniquement : `~/.config/autostart/*.desktop` (Linux), `~/Library/LaunchAgents` (macOS), clé de registre `HKEY_CURRENT_USER\...\Run` (Windows, distincte de la clé équivalente sous `HKEY_LOCAL_MACHINE` qui nécessiterait admin).

#### Gestionnaires de paquets (`src/software/packages.rs` → `Vec<PackageManagerInfo>`)
- `manager: String`, `package_count: usize` — décompte seulement (pas le détail de chaque paquet, pour éviter un rapport démesuré)
- Sondés : dpkg/apt, rpm, snap, flatpak, cargo, npm (global), brew — absent de la liste si le gestionnaire n'est pas installé.

#### Connexions réseau (`src/software/network_connections.rs` → `Vec<NetworkConnectionInfo>`, OS-specific)
- `protocol: String`, `local_address: String`, `state: String`
- Via `ss -tun` (Linux) / `netstat -an` (macOS/Windows). Ne liste que les connexions visibles par l'utilisateur courant ; la liste complète tous utilisateurs peut nécessiter root/admin selon l'OS.

#### Environnement de bureau (`src/software/desktop_env.rs` → `DesktopEnvironmentInfo`)
- `desktop: Option<String>` (`$XDG_CURRENT_DESKTOP`)
- `session_type: Option<String>` (`$XDG_SESSION_TYPE`)
- `locale: Option<String>` (`$LANG`)
- `timezone: Option<String>` — via `timedatectl` (Linux)

#### Historique des mises à jour (`src/software/update_history.rs` → `Vec<UpdateHistoryEntryInfo>`, OS-specific, limité aux 20 entrées les plus récentes)
- `date: String`, `description: String`
- Via `/var/log/apt/history.log` (Linux, lecture selon droits du fichier), `softwareupdate --history` (macOS), WMI `Win32_QuickFixEngineering` (Windows) — tous accessibles sans élévation dans le cas général.

#### Modules noyau chargés (`src/software/kernel_modules.rs` → `Vec<KernelModuleInfo>`, OS-specific)
- `name: String`, `size_bytes: u64`
- Via `/proc/modules` (Linux, lecture libre), `kextstat` (macOS), WMI `Win32_SystemDriver` (Windows, taille non disponible → `0`).

### Navigateurs (`src/browsers/mod.rs` → `Vec<BrowserInfo>`, OS-specific)
- `name: String`
- `version: Option<String>` — obtenue en exécutant `--version`
- `path: Option<String>`
- `is_default: bool`
- `extensions: Option<Vec<BrowserExtensionInfo>>` — struct déjà définie (`id: String`, `name: String`, `version: String`) mais **toujours `None`** : non implémenté actuellement.

### Métadonnées du rapport (`src/report.rs` → `SystemReport`, racine du JSON)
- `generated_at_unix: u64` — horodatage Unix de génération
- `tool_version: String` — version de l'outil (depuis `Cargo.toml`)
- `collection_warnings: Vec<String>` — messages texte libres signalant des données non récoltées (ex : UUID machine inaccessible, aucun écran/GPU/navigateur détecté)

Tout ceci est sérialisé dans `tracker_report.json` à la racine du projet.

---

## Liste 2 — Reste à implémenter (repris de `donnees.md`, Liste 2)

### Matériel / système bas niveau
- Historique d'usage CPU/mémoire dans le temps (séries temporelles au lieu d'un instantané unique).
- Fréquence et latence RAM (timings), nombre de barrettes, emplacements.
- Santé disque S.M.A.R.T. détaillée (secteurs défectueux, durée de vie estimée, cycles d'écriture SSD) — seul le statut sommaire PASSED/FAILED sur NVMe est collecté, cf. Liste 1 ; SATA/ATA nécessite généralement root.
- Courbes de refroidissement des ventilateurs et marque/modèle (vitesse RPM déjà collectée, cf. Liste 1 ; le reste vit dans les tables SMBIOS type 27, nécessite root).
- Historique/courbe de charge de la batterie (dégradation dans le temps, pas juste une valeur instantanée).
- Débit réseau instantané (Mbps) plutôt que juste les compteurs cumulés d'octets.
- Qualité de connexion Wi-Fi détaillée (bande passante) — SSID/force du signal déjà collectés, cf. Liste 1.
- Adresse IP publique (nécessiterait une requête sortante ; adresse IP locale/passerelle/DNS déjà collectés, cf. Liste 1).
- Classification fine des périphériques USB (stockage/réseau/autre, via descripteurs d'interface).
- Firmware/microcode CPU, version TPM.

### Logiciel / OS
- Historique de démarrage (crashs, temps de boot).
- Logs système récents (erreurs noyau, journaux d'événements) — nécessite généralement root pour les logs complets (`dmesg`, `journalctl` sans droits limité à la session courante).

### Données d'usage / comportement (nécessiterait suivi dans le temps)
- Temps d'utilisation par application (pas seulement instantané CPU/mémoire).
- Historique de connexion/déconnexion utilisateur.
- Fréquence de lancement des applications.

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
- **Adresse MAC des interfaces réseau** : Linux via `/sys/class/net/*/address` (lecture libre) ; Windows/macOS sans privilège particulier. Non implémenté (pas dans Liste 1).
- **IP publique** (via une requête sortante) : aucune élévation requise, mais nécessiterait une connexion réseau sortante du projet (non implémenté).

### Nécessite systématiquement root/admin
- UUID machine (`/sys/class/dmi/id/product_uuid` sur Linux).
- `dmidecode` sur Linux (BIOS bas niveau, RAM détaillée si sysfs insuffisant).
- Historique boot/crash détaillé, logs noyau complets (`dmesg` complet).
- Liste de tous les ports ouverts par tous les utilisateurs (selon OS).
- État antivirus/EDR, chiffrement de disque, règles de pare-feu détaillées.
