use crate::report::SystemReport;
use std::fmt::Write as _;

pub fn generate(report: &SystemReport) -> String {
    let mut md = String::new();

    writeln!(md, "# Rapport système").unwrap();
    writeln!(md).unwrap();
    writeln!(md, "- Généré le (timestamp Unix) : {}", report.generated_at_unix).unwrap();
    writeln!(md, "- Version de l'outil : {}", report.tool_version).unwrap();
    writeln!(md).unwrap();

    write_software(&mut md, report);
    write_hardware(&mut md, report);
    write_browsers(&mut md, report);
    write_warnings(&mut md, report);
    write_processes(&mut md, report);

    md
}

fn simple_list_section<T>(md: &mut String, title: &str, header: &str, items: &[T], row: impl Fn(&T) -> String) {
    writeln!(md, "### {title} ({})", items.len()).unwrap();
    writeln!(md).unwrap();
    if !items.is_empty() {
        writeln!(md, "{header}").unwrap();
        for item in items {
            writeln!(md, "{}", row(item)).unwrap();
        }
    }
    writeln!(md).unwrap();
}

fn write_software(md: &mut String, report: &SystemReport) {
    let software = &report.software;
    let os = &software.os;

    writeln!(md, "## Logiciel").unwrap();
    writeln!(md).unwrap();

    writeln!(md, "### Système d'exploitation").unwrap();
    writeln!(md, "| Champ | Valeur |").unwrap();
    writeln!(md, "|---|---|").unwrap();
    writeln!(md, "| Nom | {} |", opt(&os.name)).unwrap();
    writeln!(md, "| Version du noyau | {} |", opt(&os.kernel_version)).unwrap();
    writeln!(md, "| Version OS | {} |", opt(&os.os_version)).unwrap();
    writeln!(md, "| Nom d'hôte | {} |", opt(&os.host_name)).unwrap();
    writeln!(md, "| Uptime (secondes) | {} |", os.uptime_seconds).unwrap();
    writeln!(md).unwrap();

    simple_list_section(
        md,
        "Comptes utilisateurs",
        "| Nom | UID | GID | Groupes |\n|---|---|---|---|",
        &software.users,
        |u| format!("| {} | {} | {} | {} |", u.name, u.uid, u.gid, u.groups.join(", ")),
    );

    writeln!(
        md,
        "### Variables d'environnement ({}, clés sensibles rédigées)",
        software.env_vars.len()
    )
    .unwrap();
    writeln!(md, "| Clé | Valeur |").unwrap();
    writeln!(md, "|---|---|").unwrap();
    for env_var in &software.env_vars {
        writeln!(md, "| {} | {} |", env_var.key, env_var.value).unwrap();
    }
    writeln!(md).unwrap();

    simple_list_section(
        md,
        "Applications installées",
        "| Nom | Version | Éditeur | Source |\n|---|---|---|---|",
        &software.installed_apps,
        |app| {
            format!(
                "| {} | {} | {} | {} |",
                app.name,
                app.version.as_deref().unwrap_or("?"),
                app.publisher.as_deref().unwrap_or("?"),
                app.source
            )
        },
    );

    simple_list_section(
        md,
        "Runtimes de développement",
        "| Nom | Version |\n|---|---|",
        &software.dev_runtimes,
        |r| format!("| {} | {} |", r.name, r.version),
    );

    simple_list_section(
        md,
        "Services / démons",
        "| Nom | État |\n|---|---|",
        &software.services,
        |s| format!("| {} | {} |", s.name, s.status),
    );

    simple_list_section(
        md,
        "Services / démons en échec",
        "| Nom | État |\n|---|---|",
        &software.failed_services,
        |s| format!("| {} | {} |", s.name, s.status),
    );

    simple_list_section(
        md,
        "Tâches planifiées (utilisateur courant)",
        "| Nom / commande | Planification |\n|---|---|",
        &software.scheduled_tasks,
        |t| format!("| {} | {} |", t.name, t.schedule),
    );

    simple_list_section(
        md,
        "Démarrage automatique",
        "| Nom | Commande |\n|---|---|",
        &software.autostart_entries,
        |e| format!("| {} | {} |", e.name, e.command.as_deref().unwrap_or("?")),
    );

    simple_list_section(
        md,
        "Gestionnaires de paquets",
        "| Gestionnaire | Nombre de paquets |\n|---|---|",
        &software.package_managers,
        |p| format!("| {} | {} |", p.manager, p.package_count),
    );

    simple_list_section(
        md,
        "Connexions réseau (utilisateur courant)",
        "| Protocole | Adresse locale | État |\n|---|---|---|",
        &software.network_connections,
        |c| format!("| {} | {} | {} |", c.protocol, c.local_address, c.state),
    );

    simple_list_section(
        md,
        "Images Docker",
        "| Dépôt | Tag | ID image | Taille | Créée |\n|---|---|---|---|---|",
        &software.docker_images,
        |img| format!("| {} | {} | {} | {} | {} |", img.repository, img.tag, img.image_id, img.size, img.created),
    );

    simple_list_section(
        md,
        "Volumes Docker",
        "| Nom | Driver | Point de montage |\n|---|---|---|",
        &software.docker_volumes,
        |v| format!("| {} | {} | {} |", v.name, v.driver, opt(&v.mountpoint)),
    );

    simple_list_section(
        md,
        "Machines virtuelles (VirtualBox / QEMU-KVM)",
        "| Nom | Hyperviseur | État | Identifiant |\n|---|---|---|---|",
        &software.virtual_machines,
        |vm| format!("| {} | {} | {} | {} |", vm.name, vm.hypervisor, vm.state, opt(&vm.identifier)),
    );

    simple_list_section(
        md,
        "Images Podman",
        "| Dépôt | Tag | ID image | Taille | Créée |\n|---|---|---|---|---|",
        &software.podman_images,
        |img| format!("| {} | {} | {} | {} | {} |", img.repository, img.tag, img.image_id, img.size, img.created),
    );

    simple_list_section(
        md,
        "Volumes Podman",
        "| Nom | Driver | Point de montage |\n|---|---|---|",
        &software.podman_volumes,
        |v| format!("| {} | {} | {} |", v.name, v.driver, opt(&v.mountpoint)),
    );

    simple_list_section(
        md,
        "Clés SSH (métadonnées uniquement, clés publiques)",
        "| Fichier | Type | Empreinte |\n|---|---|---|",
        &software.ssh_keys,
        |k| format!("| {} | {} | {} |", k.file_name, opt(&k.key_type), opt(&k.fingerprint)),
    );

    writeln!(md, "### Configuration proxy système").unwrap();
    match &software.proxy_config {
        Some(proxy) => {
            writeln!(md, "| Champ | Valeur |").unwrap();
            writeln!(md, "|---|---|").unwrap();
            writeln!(md, "| HTTP | {} |", opt(&proxy.http_proxy)).unwrap();
            writeln!(md, "| HTTPS | {} |", opt(&proxy.https_proxy)).unwrap();
            writeln!(md, "| Exceptions (no_proxy) | {} |", opt(&proxy.no_proxy)).unwrap();
            writeln!(md, "| Source | {} |", proxy.source).unwrap();
        }
        None => {
            writeln!(md, "Aucun proxy configuré détecté.").unwrap();
        }
    }
    writeln!(md).unwrap();

    writeln!(md, "### Polices installées ({})", software.fonts.total_count).unwrap();
    if !software.fonts.families.is_empty() {
        writeln!(md, "{}", software.fonts.families.join(", ")).unwrap();
    }
    writeln!(md).unwrap();

    writeln!(md, "### Environnement de bureau").unwrap();
    writeln!(md, "| Champ | Valeur |").unwrap();
    writeln!(md, "|---|---|").unwrap();
    writeln!(md, "| Environnement | {} |", opt(&software.desktop_environment.desktop)).unwrap();
    writeln!(md, "| Type de session | {} |", opt(&software.desktop_environment.session_type)).unwrap();
    writeln!(md, "| Locale | {} |", opt(&software.desktop_environment.locale)).unwrap();
    writeln!(md, "| Fuseau horaire | {} |", opt(&software.desktop_environment.timezone)).unwrap();
    writeln!(md).unwrap();

    simple_list_section(
        md,
        "Historique des mises à jour",
        "| Date | Description |\n|---|---|",
        &software.update_history,
        |u| format!("| {} | {} |", u.date, u.description),
    );

    simple_list_section(
        md,
        "Modules noyau chargés",
        "| Nom | Taille (octets) |\n|---|---|",
        &software.kernel_modules,
        |m| format!("| {} | {} |", m.name, m.size_bytes),
    );
}

fn write_hardware(md: &mut String, report: &SystemReport) {
    let hardware = &report.hardware;

    writeln!(md, "## Matériel").unwrap();
    writeln!(md).unwrap();

    writeln!(md, "### Mémoire").unwrap();
    writeln!(md, "| Champ | Valeur |").unwrap();
    writeln!(md, "|---|---|").unwrap();
    writeln!(md, "| RAM totale | {} Mo |", hardware.memory.total_mb).unwrap();
    writeln!(md, "| RAM utilisée | {} Mo |", hardware.memory.used_mb).unwrap();
    writeln!(md, "| Swap total | {} Mo |", hardware.memory.total_swap_mb).unwrap();
    writeln!(md, "| Swap utilisé | {} Mo |", hardware.memory.used_swap_mb).unwrap();
    writeln!(md).unwrap();

    writeln!(md, "### CPU").unwrap();
    writeln!(md, "- Architecture : {}", hardware.cpu.architecture).unwrap();
    writeln!(md, "- Nombre de cœurs : {}", hardware.cpu.core_count).unwrap();
    writeln!(
        md,
        "- Utilisation globale : {:.1}%",
        hardware.cpu.global_usage_percent
    )
    .unwrap();
    writeln!(md).unwrap();
    writeln!(md, "| Cœur | Usage % | Fréquence (MHz) | Marque |").unwrap();
    writeln!(md, "|---|---|---|---|").unwrap();
    for core in &hardware.cpu.cores {
        writeln!(
            md,
            "| {} | {:.1} | {} | {} |",
            core.index, core.usage_percent, core.frequency_mhz, core.brand
        )
        .unwrap();
    }
    writeln!(md).unwrap();
    writeln!(md, "- Gouverneur de fréquence : {}", opt(&hardware.cpu.scaling_governor)).unwrap();
    writeln!(md).unwrap();

    simple_list_section(
        md,
        "Vulnérabilités CPU connues (mitigations)",
        "| Nom | Statut |\n|---|---|",
        &hardware.cpu.vulnerabilities,
        |v| format!("| {} | {} |", v.name, v.status),
    );

    simple_list_section(
        md,
        "Stockage",
        "| Nom | Type | Système de fichiers | Point de montage | Amovible | Utilisé / Total (Go) | Santé SMART |\n|---|---|---|---|---|---|---|",
        &hardware.disks,
        |disk| {
            format!(
                "| {} | {} | {} | {} | {} | {} / {} | {} |",
                disk.name,
                disk.kind,
                disk.file_system,
                disk.mount_point,
                if disk.is_removable { "Oui" } else { "Non" },
                disk.used_gb,
                disk.total_gb,
                opt(&disk.smart_health)
            )
        },
    );

    simple_list_section(
        md,
        "Stockage virtuel (overlay Docker/containerd, etc.)",
        "| Nom | Système de fichiers | Point de montage |\n|---|---|---|",
        &hardware.virtual_disks,
        |disk| format!("| {} | {} | {} |", disk.name, disk.file_system, disk.mount_point),
    );

    writeln!(md, "### Réseau ({} interface(s))", hardware.network.interfaces.len()).unwrap();
    writeln!(
        md,
        "| Interface | Reçu (octets) | Émis (octets) | MAC | IPv4 | IPv6 | Vitesse (Mbps) | Type |"
    )
    .unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|---|").unwrap();
    for network in &hardware.network.interfaces {
        writeln!(
            md,
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            network.interface_name,
            network.received_bytes,
            network.transmitted_bytes,
            opt(&network.mac_address),
            if network.ipv4_addresses.is_empty() { "?".to_string() } else { network.ipv4_addresses.join(", ") },
            if network.ipv6_addresses.is_empty() { "?".to_string() } else { network.ipv6_addresses.join(", ") },
            network.link_speed_mbps.map(|v| v.to_string()).unwrap_or_else(|| "?".to_string()),
            opt(&network.connection_type),
        )
        .unwrap();
    }
    writeln!(md).unwrap();
    writeln!(md, "- Passerelle par défaut : {}", opt(&hardware.network.default_gateway)).unwrap();
    writeln!(md, "- Serveurs DNS : {}", if hardware.network.dns_servers.is_empty() { "?".to_string() } else { hardware.network.dns_servers.join(", ") }).unwrap();
    writeln!(md).unwrap();

    simple_list_section(
        md,
        "Wi-Fi",
        "| SSID | Signal (%) | Interface |\n|---|---|---|",
        &hardware.wifi,
        |w| {
            format!(
                "| {} | {} | {} |",
                w.ssid,
                w.signal_percent.map(|s| s.to_string()).unwrap_or_else(|| "?".to_string()),
                w.interface.as_deref().unwrap_or("?")
            )
        },
    );

    simple_list_section(
        md,
        "Périphériques PCI",
        "| Nom | Classe |\n|---|---|",
        &hardware.pci_devices,
        |p| format!("| {} | {} |", p.name, p.class),
    );

    simple_list_section(
        md,
        "Capteurs / composants",
        "| Label | Température (°C) | Max (°C) | Critique (°C) |\n|---|---|---|---|",
        &hardware.components,
        |component| {
            format!(
                "| {} | {} | {} | {} |",
                component.label,
                opt_num(component.temperature_celsius),
                opt_num(component.max_temperature_celsius),
                opt_num(component.critical_temperature_celsius)
            )
        },
    );

    writeln!(md, "### Batterie(s) ({})", hardware.batteries.len()).unwrap();
    if !hardware.batteries.is_empty() {
        writeln!(
            md,
            "| Fabricant | Modèle | État | Technologie | Charge % | Santé % | Cycles |"
        )
        .unwrap();
        writeln!(md, "|---|---|---|---|---|---|---|").unwrap();
        for battery in &hardware.batteries {
            writeln!(
                md,
                "| {} | {} | {} | {} | {:.1} | {:.1} | {} |",
                battery.vendor.as_deref().unwrap_or("?"),
                battery.model.as_deref().unwrap_or("?"),
                battery.state,
                battery.technology,
                battery.state_of_charge_percent,
                battery.state_of_health_percent,
                opt_num(battery.cycle_count.map(|c| c as f32))
            )
            .unwrap();
        }
    }
    writeln!(md).unwrap();

    writeln!(md, "### Carte mère / BIOS").unwrap();
    writeln!(md, "| Champ | Valeur |").unwrap();
    writeln!(md, "|---|---|").unwrap();
    writeln!(md, "| Fabricant | {} |", opt(&hardware.motherboard.vendor)).unwrap();
    writeln!(md, "| Modèle | {} |", opt(&hardware.motherboard.model)).unwrap();
    writeln!(md, "| Version | {} |", opt(&hardware.motherboard.version)).unwrap();
    writeln!(
        md,
        "| Fabricant BIOS | {} |",
        opt(&hardware.motherboard.bios_vendor)
    )
    .unwrap();
    writeln!(
        md,
        "| Version BIOS | {} |",
        opt(&hardware.motherboard.bios_version)
    )
    .unwrap();
    writeln!(md, "| Date BIOS | {} |", opt(&hardware.motherboard.bios_date)).unwrap();
    writeln!(
        md,
        "| UUID machine | {} |",
        opt(&hardware.motherboard.machine_uuid)
    )
    .unwrap();
    writeln!(md, "| Secure Boot | {} |", opt(&hardware.motherboard.secure_boot)).unwrap();
    writeln!(md).unwrap();

    simple_list_section(
        md,
        "GPU(s)",
        "| Nom | Fabricant | VRAM (Mo) | Version driver |\n|---|---|---|---|",
        &hardware.gpus,
        |gpu| {
            format!(
                "| {} | {} | {} | {} |",
                gpu.name,
                opt(&gpu.vendor),
                gpu.vram_mb.map(|v| v.to_string()).unwrap_or_else(|| "?".to_string()),
                opt(&gpu.driver_version)
            )
        },
    );

    simple_list_section(
        md,
        "Écran(s)",
        "| Nom | Résolution | Position | Échelle | Fréquence (Hz) | Primaire | Intégré |\n|---|---|---|---|---|---|---|",
        &hardware.monitors,
        |monitor| {
            format!(
                "| {} | {}x{} | ({}, {}) | {:.2} | {:.0} | {} | {} |",
                monitor.name,
                monitor.width,
                monitor.height,
                monitor.x,
                monitor.y,
                monitor.scale_factor,
                monitor.frequency_hz,
                if monitor.is_primary { "Oui" } else { "Non" },
                if monitor.is_builtin { "Oui" } else { "Non" }
            )
        },
    );

    simple_list_section(
        md,
        "Lecteurs optiques / disquettes",
        "| Nom | Fabricant | Type |\n|---|---|---|",
        &hardware.optical_drives,
        |drive| {
            format!(
                "| {} | {} | {} |",
                drive.name,
                drive.vendor.as_deref().unwrap_or("?"),
                drive.kind
            )
        },
    );

    simple_list_section(
        md,
        "Périphériques",
        "| Nom | Type |\n|---|---|",
        &hardware.peripherals,
        |peripheral| format!("| {} | {} |", peripheral.name, peripheral.kind),
    );

    simple_list_section(
        md,
        "Souris",
        "| Nom |\n|---|",
        &hardware.mice,
        |mouse| format!("| {} |", mouse.name),
    );

    simple_list_section(
        md,
        "Manette(s)",
        "| Nom |\n|---|",
        &hardware.gamepads,
        |gamepad| format!("| {} |", gamepad.name),
    );

    simple_list_section(
        md,
        "Touchpad(s)",
        "| Nom |\n|---|",
        &hardware.touchpads,
        |touchpad| format!("| {} |", touchpad.name),
    );

    simple_list_section(
        md,
        "Caméra(s)",
        "| Nom |\n|---|",
        &hardware.cameras,
        |camera| format!("| {} |", camera.name),
    );

    simple_list_section(
        md,
        "Périphériques externes (USB)",
        "| Nom | Fabricant |\n|---|---|",
        &hardware.usb_devices,
        |device| format!("| {} | {} |", device.name, device.vendor.as_deref().unwrap_or("?")),
    );

    simple_list_section(
        md,
        "Périphériques Bluetooth appairés",
        "| Nom |\n|---|",
        &hardware.bluetooth_devices,
        |device| format!("| {} |", device.name),
    );

    simple_list_section(
        md,
        "Imprimantes / Scanners",
        "| Nom | Type |\n|---|---|",
        &hardware.printers,
        |printer| format!("| {} | {} |", printer.name, printer.kind),
    );

    simple_list_section(
        md,
        "Ventilateur(s)",
        "| Nom | Vitesse (tr/min) |\n|---|---|",
        &hardware.fans,
        |fan| {
            format!(
                "| {} | {} |",
                fan.name,
                fan.speed_rpm.map(|v| v.to_string()).unwrap_or_else(|| "?".to_string())
            )
        },
    );

    simple_list_section(
        md,
        "Partitions",
        "| Périphérique | Système de fichiers | Taille (Go) |\n|---|---|---|",
        &hardware.storage_layout.partitions,
        |p| format!("| {} | {} | {} |", p.device, p.fs_type, p.size_gb),
    );

    simple_list_section(
        md,
        "Volumes LVM",
        "| Groupe de volumes | Volume logique | Taille (Go) |\n|---|---|---|",
        &hardware.storage_layout.lvm_volumes,
        |v| format!("| {} | {} | {} |", v.vg_name, v.lv_name, v.size_gb),
    );

    simple_list_section(
        md,
        "Tableaux RAID logiciels",
        "| Périphérique | Niveau | État | Membres |\n|---|---|---|---|",
        &hardware.storage_layout.raid_arrays,
        |r| format!("| {} | {} | {} | {} |", r.device, r.level, r.state, r.devices.join(", ")),
    );

    writeln!(md, "### Profil d'alimentation").unwrap();
    writeln!(md, "| Champ | Valeur |").unwrap();
    writeln!(md, "|---|---|").unwrap();
    writeln!(md, "| Profil | {} |", opt(&hardware.power_profile.profile)).unwrap();
    writeln!(md, "| Mode de veille | {} |", opt(&hardware.power_profile.sleep_mode)).unwrap();
    writeln!(md).unwrap();
}

fn write_browsers(md: &mut String, report: &SystemReport) {
    writeln!(md, "## Navigateurs ({})", report.browsers.len()).unwrap();
    writeln!(md).unwrap();
    if report.browsers.is_empty() {
        writeln!(md, "Aucun navigateur détecté.").unwrap();
    } else {
        writeln!(md, "| Nom | Version | Par défaut | Chemin |").unwrap();
        writeln!(md, "|---|---|---|---|").unwrap();
        for browser in &report.browsers {
            writeln!(
                md,
                "| {} | {} | {} | {} |",
                browser.name,
                browser.version.as_deref().unwrap_or("inconnue"),
                if browser.is_default { "Oui" } else { "Non" },
                browser.path.as_deref().unwrap_or("?")
            )
            .unwrap();
        }
    }
    writeln!(md).unwrap();
}

fn write_warnings(md: &mut String, report: &SystemReport) {
    if report.collection_warnings.is_empty() {
        return;
    }
    writeln!(md, "## Avertissements de collecte").unwrap();
    writeln!(md).unwrap();
    for warning in &report.collection_warnings {
        writeln!(md, "- {warning}").unwrap();
    }
    writeln!(md).unwrap();
}

fn write_processes(md: &mut String, report: &SystemReport) {
    let processes = &report.software.processes;

    writeln!(md, "## Processus ({} au total)", processes.total_count).unwrap();
    writeln!(md).unwrap();
    if !processes.processes.is_empty() {
        writeln!(md, "| PID | Nom | CPU % | Mémoire (Mo) |").unwrap();
        writeln!(md, "|---|---|---|---|").unwrap();
        for process in &processes.processes {
            writeln!(
                md,
                "| {} | {} | {:.2} | {} |",
                process.pid, process.name, process.cpu_usage_percent, process.memory_mb
            )
            .unwrap();
        }
    }
    writeln!(md).unwrap();
}

fn opt(value: &Option<String>) -> String {
    value.clone().unwrap_or_else(|| "?".to_string())
}

fn opt_num(value: Option<f32>) -> String {
    value.map(|v| format!("{v:.1}")).unwrap_or_else(|| "?".to_string())
}
