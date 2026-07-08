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
        let separator_cols = header.matches('|').count().saturating_sub(1);
        writeln!(md, "|{}", "---|".repeat(separator_cols)).unwrap();
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

    writeln!(md, "### Comptes utilisateurs ({})", software.users.len()).unwrap();
    writeln!(md, "| Nom | UID | GID | Groupes |").unwrap();
    writeln!(md, "|---|---|---|---|").unwrap();
    for user in &software.users {
        writeln!(
            md,
            "| {} | {} | {} | {} |",
            user.name,
            user.uid,
            user.gid,
            user.groups.join(", ")
        )
        .unwrap();
    }
    writeln!(md).unwrap();

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

    writeln!(
        md,
        "### Applications installées ({})",
        software.installed_apps.len()
    )
    .unwrap();
    writeln!(md, "| Nom | Version | Éditeur | Source |").unwrap();
    writeln!(md, "|---|---|---|---|").unwrap();
    for app in &software.installed_apps {
        writeln!(
            md,
            "| {} | {} | {} | {} |",
            app.name,
            app.version.as_deref().unwrap_or("?"),
            app.publisher.as_deref().unwrap_or("?"),
            app.source
        )
        .unwrap();
    }
    writeln!(md).unwrap();

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

    writeln!(md, "### Stockage ({} disque(s))", hardware.disks.len()).unwrap();
    writeln!(
        md,
        "| Nom | Type | Système de fichiers | Point de montage | Amovible | Utilisé / Total (Go) | Santé SMART |"
    )
    .unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|").unwrap();
    for disk in &hardware.disks {
        writeln!(
            md,
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
        .unwrap();
    }
    writeln!(md).unwrap();

    writeln!(
        md,
        "### Stockage virtuel ({} montage(s) : overlay Docker/containerd, etc.)",
        hardware.virtual_disks.len()
    )
    .unwrap();
    if !hardware.virtual_disks.is_empty() {
        writeln!(md, "| Nom | Système de fichiers | Point de montage |").unwrap();
        writeln!(md, "|---|---|---|").unwrap();
        for disk in &hardware.virtual_disks {
            writeln!(
                md,
                "| {} | {} | {} |",
                disk.name, disk.file_system, disk.mount_point
            )
            .unwrap();
        }
    }
    writeln!(md).unwrap();

    writeln!(md, "### Réseau ({} interface(s))", hardware.network.interfaces.len()).unwrap();
    writeln!(md, "| Interface | Reçu (octets) | Émis (octets) |").unwrap();
    writeln!(md, "|---|---|---|").unwrap();
    for network in &hardware.network.interfaces {
        writeln!(
            md,
            "| {} | {} | {} |",
            network.interface_name, network.received_bytes, network.transmitted_bytes
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

    writeln!(
        md,
        "### Capteurs / composants ({})",
        hardware.components.len()
    )
    .unwrap();
    writeln!(md, "| Label | Température (°C) | Max (°C) | Critique (°C) |").unwrap();
    writeln!(md, "|---|---|---|---|").unwrap();
    for component in &hardware.components {
        writeln!(
            md,
            "| {} | {} | {} | {} |",
            component.label,
            opt_num(component.temperature_celsius),
            opt_num(component.max_temperature_celsius),
            opt_num(component.critical_temperature_celsius)
        )
        .unwrap();
    }
    writeln!(md).unwrap();

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

    writeln!(md, "### GPU(s) ({})", hardware.gpus.len()).unwrap();
    writeln!(md, "| Nom | Fabricant |").unwrap();
    writeln!(md, "|---|---|").unwrap();
    for gpu in &hardware.gpus {
        writeln!(md, "| {} | {} |", gpu.name, opt(&gpu.vendor)).unwrap();
    }
    writeln!(md).unwrap();

    writeln!(md, "### Écran(s) ({})", hardware.monitors.len()).unwrap();
    writeln!(
        md,
        "| Nom | Résolution | Position | Échelle | Fréquence (Hz) | Primaire | Intégré |"
    )
    .unwrap();
    writeln!(md, "|---|---|---|---|---|---|---|").unwrap();
    for monitor in &hardware.monitors {
        writeln!(
            md,
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
        .unwrap();
    }
    writeln!(md).unwrap();

    writeln!(
        md,
        "### Lecteurs optiques / disquettes ({})",
        hardware.optical_drives.len()
    )
    .unwrap();
    if !hardware.optical_drives.is_empty() {
        writeln!(md, "| Nom | Fabricant | Type |").unwrap();
        writeln!(md, "|---|---|---|").unwrap();
        for drive in &hardware.optical_drives {
            writeln!(
                md,
                "| {} | {} | {} |",
                drive.name,
                drive.vendor.as_deref().unwrap_or("?"),
                drive.kind
            )
            .unwrap();
        }
    }
    writeln!(md).unwrap();

    writeln!(md, "### Périphériques ({})", hardware.peripherals.len()).unwrap();
    if !hardware.peripherals.is_empty() {
        writeln!(md, "| Nom | Type |").unwrap();
        writeln!(md, "|---|---|").unwrap();
        for peripheral in &hardware.peripherals {
            writeln!(md, "| {} | {} |", peripheral.name, peripheral.kind).unwrap();
        }
    }
    writeln!(md).unwrap();

    writeln!(md, "### Souris ({})", hardware.mice.len()).unwrap();
    if !hardware.mice.is_empty() {
        writeln!(md, "| Nom |").unwrap();
        writeln!(md, "|---|").unwrap();
        for mouse in &hardware.mice {
            writeln!(md, "| {} |", mouse.name).unwrap();
        }
    }
    writeln!(md).unwrap();

    writeln!(md, "### Manette(s) ({})", hardware.gamepads.len()).unwrap();
    if !hardware.gamepads.is_empty() {
        writeln!(md, "| Nom |").unwrap();
        writeln!(md, "|---|").unwrap();
        for gamepad in &hardware.gamepads {
            writeln!(md, "| {} |", gamepad.name).unwrap();
        }
    }
    writeln!(md).unwrap();

    writeln!(md, "### Touchpad(s) ({})", hardware.touchpads.len()).unwrap();
    if !hardware.touchpads.is_empty() {
        writeln!(md, "| Nom |").unwrap();
        writeln!(md, "|---|").unwrap();
        for touchpad in &hardware.touchpads {
            writeln!(md, "| {} |", touchpad.name).unwrap();
        }
    }
    writeln!(md).unwrap();

    writeln!(md, "### Caméra(s) ({})", hardware.cameras.len()).unwrap();
    if !hardware.cameras.is_empty() {
        writeln!(md, "| Nom |").unwrap();
        writeln!(md, "|---|").unwrap();
        for camera in &hardware.cameras {
            writeln!(md, "| {} |", camera.name).unwrap();
        }
    }
    writeln!(md).unwrap();

    writeln!(
        md,
        "### Périphériques externes (USB) ({})",
        hardware.usb_devices.len()
    )
    .unwrap();
    if !hardware.usb_devices.is_empty() {
        writeln!(md, "| Nom | Fabricant |").unwrap();
        writeln!(md, "|---|---|").unwrap();
        for device in &hardware.usb_devices {
            writeln!(
                md,
                "| {} | {} |",
                device.name,
                device.vendor.as_deref().unwrap_or("?")
            )
            .unwrap();
        }
    }
    writeln!(md).unwrap();

    writeln!(
        md,
        "### Périphériques Bluetooth appairés ({})",
        hardware.bluetooth_devices.len()
    )
    .unwrap();
    if !hardware.bluetooth_devices.is_empty() {
        writeln!(md, "| Nom |").unwrap();
        writeln!(md, "|---|").unwrap();
        for device in &hardware.bluetooth_devices {
            writeln!(md, "| {} |", device.name).unwrap();
        }
    }
    writeln!(md).unwrap();

    writeln!(
        md,
        "### Imprimantes / Scanners ({})",
        hardware.printers.len()
    )
    .unwrap();
    if !hardware.printers.is_empty() {
        writeln!(md, "| Nom | Type |").unwrap();
        writeln!(md, "|---|---|").unwrap();
        for printer in &hardware.printers {
            writeln!(md, "| {} | {} |", printer.name, printer.kind).unwrap();
        }
    }
    writeln!(md).unwrap();

    writeln!(md, "### Ventilateur(s) ({})", hardware.fans.len()).unwrap();
    if !hardware.fans.is_empty() {
        writeln!(md, "| Nom | Vitesse (tr/min) |").unwrap();
        writeln!(md, "|---|---|").unwrap();
        for fan in &hardware.fans {
            writeln!(
                md,
                "| {} | {} |",
                fan.name,
                fan.speed_rpm.map(|v| v.to_string()).unwrap_or_else(|| "?".to_string())
            )
            .unwrap();
        }
    }
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
