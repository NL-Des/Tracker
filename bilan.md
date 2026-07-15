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

#### Réseau (`src/hardware/network.rs` → `NetworkInfo`, sous-modules OS pour les détails par interface)
- `interfaces: Vec<NetworkInterfaceInfo>` :
  - `interface_name: String`
  - `received_bytes: u64` — cumulé depuis le démarrage
  - `transmitted_bytes: u64` — cumulé depuis le démarrage
  - `mac_address: Option<String>` — Linux via `/sys/class/net/<iface>/address` ; Windows via WMI `Win32_NetworkAdapterConfiguration` ; macOS via `ifconfig` (`ether`). Lecture libre.
  - `ipv4_addresses: Vec<String>` / `ipv6_addresses: Vec<String>` — Linux via `ip -o addr show` ; Windows via WMI `Win32_NetworkAdapterConfiguration.IPAddress` ; macOS via `ifconfig` (`inet`/`inet6`).
  - `link_speed_mbps: Option<u64>` — Linux via `/sys/class/net/<iface>/speed` (souvent `None` si interface down) ; Windows via WMI `Win32_NetworkAdapter.Speed`. Non implémenté sur macOS (pas de source lecture-libre identifiée).
  - `connection_type: Option<String>` — `"wired"`/`"wifi"`/`"loopback"`/`"virtual"`, Linux via présence de `/sys/class/net/<iface>/wireless` ; Windows via WMI `Win32_NetworkAdapter.AdapterType`. Non implémenté sur macOS.
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
- `vram_mb: Option<u64>` — Linux via sysfs `mem_info_vram_total` (AMD) avec repli sur `nvidia-smi` (NVIDIA propriétaire) ; Windows via WMI `Win32_VideoController.AdapterRAM` (peu fiable au-delà de 4 Go, limitation connue du champ) ; macOS via `system_profiler SPDisplaysDataType` (clé "VRAM (Total)").
- `driver_version: Option<String>` — Linux via `nvidia-smi` uniquement (pas d'équivalent sysfs générique) ; Windows via WMI `Win32_VideoController.DriverVersion` ; macOS : pas de numéro de version driver classique, le support Metal ("Metal Support") est utilisé comme information la plus proche disponible.

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

#### Disposition du stockage (`src/hardware/storage_layout.rs` → `StorageLayoutInfo`, OS-specific)
- `partitions: Vec<PartitionInfo>` (`device`, `fs_type`, `size_gb`) — Linux via `lsblk -J -b` ; macOS via `diskutil list` (parsing heuristique) ; Windows via WMI `Win32_LogicalDisk`. Complète les points de montage déjà collectés par `disks.rs`.
- `lvm_volumes: Vec<LvmVolumeInfo>` (`vg_name`, `lv_name`, `size_gb`) — Linux via `lvs`, absent si LVM non installé. Pas d'équivalent macOS/Windows.
- `raid_arrays: Vec<RaidArrayInfo>` (`device`, `level`, `state`, `devices`) — Linux via `/proc/mdstat` (RAID logiciel `mdadm`, lecture libre). Pas d'équivalent macOS/Windows dans ce périmètre.

#### Profil d'alimentation (`src/hardware/power_profile.rs` → `PowerProfileInfo`, OS-specific)
- `profile: Option<String>` — Linux via `powerprofilesctl get` (démon `power-profiles-daemon`, absent sur certaines distributions) ; Windows via `powercfg /getactivescheme`. Pas de notion de profil nommé sur macOS.
- `sleep_mode: Option<String>` — Linux via `/sys/power/mem_sleep` (mode entre crochets) ; macOS via `pmset -g` (`hibernatemode`, information la plus proche disponible). Non implémenté sur Windows.

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
- `collect_failed()` (champ `failed_services` dans `SoftwareInfo`) : unités en échec uniquement, via `systemctl --failed` (Linux). Pas de notion équivalente stricte sur macOS/Windows → `Vec` vide.

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

#### Images Docker (`src/software/docker.rs` → `Vec<DockerImageInfo>`, OS-specific)
- `repository: String`, `tag: String`, `image_id: String`, `size: String`, `created: String`
- Via le CLI `docker` (ex : `docker images`). Infaillible par design : absence du CLI ou erreur d'accès renvoie un `Vec` vide. Lecture seule, aucune élévation requise.

#### Volumes Docker (`src/software/docker.rs` → `Vec<DockerVolumeInfo>`, OS-specific)
- `name: String`, `driver: String`, `mountpoint: Option<String>`
- Via le CLI `docker` (ex : `docker volume ls`/`inspect`). Mêmes garanties que les images Docker ci-dessus.

#### Machines virtuelles (`src/software/virtual_machines.rs` → `Vec<VirtualMachineInfo>`, OS-specific)
- `name: String`, `hypervisor: String`, `state: String`, `identifier: Option<String>`
- Via `VBoxManage` (VirtualBox) et `virsh` (libvirt/KVM) selon disponibilité. Infaillible par design : absence d'outil ou erreur d'accès renvoie un `Vec` vide. Lecture seule, aucune élévation requise.

#### Images/volumes Podman (`src/software/podman.rs` → `Vec<DockerImageInfo>`/`Vec<DockerVolumeInfo>`, mêmes structs que Docker)
- Réutilise les structs `DockerImageInfo`/`DockerVolumeInfo` définies dans `docker.rs` (le CLI `podman` a une surface quasi identique à `docker`). Exposé sous `podman_images`/`podman_volumes` dans `SoftwareInfo`.
- Containerd (`ctr`) volontairement non couvert : son socket nécessite généralement root, ce qui sortirait du périmètre "sans droits admin" du projet.

#### Polices installées (`src/software/fonts.rs` → `FontsSummary`)
- `total_count: usize`, `families: Vec<String>` (dédoublonnées, triées)
- Via `fc-list : family` (fontconfig, Linux et macOS si installé) ; Windows via énumération des valeurs de la clé de registre `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts` (lecture, pas d'admin requis).

#### Configuration proxy système (`src/software/proxy_config.rs` → `Option<ProxyConfigInfo>`)
- `http_proxy: Option<String>`, `https_proxy: Option<String>`, `no_proxy: Option<String>`, `source: String` (`"env"`/`"gsettings"`/`"scutil"`/`"registry"`)
- Vérifie d'abord les variables d'environnement standard (`HTTP_PROXY`/`HTTPS_PROXY`/`NO_PROXY`), puis retombe sur la config système : `gsettings` (GNOME, Linux), `scutil --proxy` (macOS), registre `HKEY_CURRENT_USER\...\Internet Settings` (Windows, HKCU donc pas d'admin requis). `None` si aucune configuration détectée.

#### Clés SSH (`src/software/ssh_keys.rs` → `Vec<SshKeyInfo>`, cross-plateforme)
- `file_name: String`, `key_type: Option<String>`, `fingerprint: Option<String>`
- Scanne uniquement `~/.ssh/*.pub` (jamais le contenu d'une clé privée). Type extrait du contenu du fichier public, empreinte via `ssh-keygen -lf`.

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
- **Vitesse de liaison réseau et type de connexion sur macOS** — implémentés sur Linux/Windows (cf. Liste 1), pas de source lecture-libre identifiée sur macOS pour l'instant.
- **RAID logiciel / LVM sur macOS et Windows** — implémentés sur Linux (`/proc/mdstat`, `lvs`), pas d'équivalent direct identifié sur les deux autres OS (Storage Spaces sur Windows nécessiterait des classes WMI plus complexes).

### Logiciel / OS
- Historique de démarrage (crashs, temps de boot).
- Logs système récents (erreurs noyau, journaux d'événements) — nécessite généralement root pour les logs complets (`dmesg`, `journalctl` sans droits limité à la session courante).
- **Containerd** (`ctr`/`nerdctl`) — volontairement non couvert : le socket containerd nécessite généralement root, hors périmètre "sans droits admin" du projet (Docker et Podman sont couverts, cf. Liste 1).

> Volontairement exclu du périmètre pour rester non intrusif : historique du presse-papiers, liste des fichiers récemment ouverts, tout suivi fin de l'usage applicatif au-delà d'un instantané.

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
- **Adresse MAC des interfaces réseau** : Linux via `/sys/class/net/*/address` (lecture libre) ; Windows/macOS sans privilège particulier — implémenté, cf. Liste 1.
- **IP publique** (via une requête sortante) : aucune élévation requise, mais nécessiterait une connexion réseau sortante du projet (non implémenté).

### Nécessite systématiquement root/admin
- UUID machine (`/sys/class/dmi/id/product_uuid` sur Linux).
- `dmidecode` sur Linux (BIOS bas niveau, RAM détaillée si sysfs insuffisant).
- Historique boot/crash détaillé, logs noyau complets (`dmesg` complet).
- Liste de tous les ports ouverts par tous les utilisateurs (selon OS).
- État antivirus/EDR, chiffrement de disque, règles de pare-feu détaillées.
