use crate::report::SystemReport;

pub fn print_report(report: &SystemReport) {
    println!("--- System Data ---");

    println!("\n=== SOFTWARE ===");
    print_software(report);

    println!("\n=== HARDWARE ===");
    print_hardware(report);

    println!("\n=== BROWSERS ===");
    print_browsers(report);

    if !report.collection_warnings.is_empty() {
        println!("\n=== Avertissements de collecte ===");
        for warning in &report.collection_warnings {
            println!("  - {warning}");
        }
    }
}

fn print_software(report: &SystemReport) {
    let software = &report.software;
    let os = &software.os;

    println!("System name:             {:?}", os.name);
    println!("System kernel version:   {:?}", os.kernel_version);
    println!("System OS version:       {:?}", os.os_version);
    println!("System host name:        {:?}", os.host_name);
    println!("Ordinateur allumé depuis : {} secondes", os.uptime_seconds);

    println!(
        "Nombre de processus en cours : {}",
        software.processes.total_count
    );
    println!("=> Liste des processus :");
    for process in &software.processes.processes {
        println!(
            "PID: {} | Nom: {} | CPU: {:.2}% | Mémoire: {} Mo",
            process.pid, process.name, process.cpu_usage_percent, process.memory_mb
        );
    }

    println!("=> Utilisateurs système : {} compte(s)", software.users.len());
    for user in &software.users {
        println!("  {} (uid: {}, gid: {})", user.name, user.uid, user.gid);
    }

    println!(
        "=> Variables d'environnement : {} (clés sensibles rédigées)",
        software.env_vars.len()
    );

    println!(
        "=> Logiciels installés détectés : {}",
        software.installed_apps.len()
    );
    for app in software.installed_apps.iter().take(10) {
        println!(
            "  {} {} ({})",
            app.name,
            app.version.as_deref().unwrap_or("?"),
            app.source
        );
    }
    if software.installed_apps.len() > 10 {
        println!("  ... et {} de plus", software.installed_apps.len() - 10);
    }

    println!("=> Runtimes de développement détectés : {}", software.dev_runtimes.len());
    for runtime in &software.dev_runtimes {
        println!("  {} : {}", runtime.name, runtime.version);
    }

    println!("=> Services/démons : {}", software.services.len());

    println!("=> Tâches planifiées (utilisateur courant) : {}", software.scheduled_tasks.len());
    for task in &software.scheduled_tasks {
        println!("  [{}] {}", task.schedule, task.name);
    }

    println!("=> Éléments de démarrage automatique : {}", software.autostart_entries.len());
    for entry in &software.autostart_entries {
        println!("  {} ({:?})", entry.name, entry.command);
    }

    println!("=> Gestionnaires de paquets détectés :");
    for manager in &software.package_managers {
        println!("  {} : {} paquet(s)", manager.manager, manager.package_count);
    }

    println!("=> Connexions réseau (utilisateur courant) : {}", software.network_connections.len());

    println!(
        "=> Environnement de bureau : {:?} | Session : {:?} | Locale : {:?} | Fuseau horaire : {:?}",
        software.desktop_environment.desktop,
        software.desktop_environment.session_type,
        software.desktop_environment.locale,
        software.desktop_environment.timezone
    );

    println!("=> Historique des mises à jour : {} entrée(s)", software.update_history.len());

    println!("=> Modules noyau chargés : {}", software.kernel_modules.len());
}

fn print_hardware(report: &SystemReport) {
    let hardware = &report.hardware;

    println!(
        "total memory: {} Mo | used memory: {} Mo",
        hardware.memory.total_mb, hardware.memory.used_mb
    );
    println!(
        "total swap: {} Mo | used swap: {} Mo",
        hardware.memory.total_swap_mb, hardware.memory.used_swap_mb
    );

    println!("Architecture CPU: {}", hardware.cpu.architecture);
    println!("NB CPUs: {}", hardware.cpu.core_count);
    println!(
        "Utilisation globale du CPU : {}%",
        hardware.cpu.global_usage_percent
    );
    for core in &hardware.cpu.cores {
        println!(
            "Cœur #{} : {}% | Fréquence : {} MHz | Marque : {}",
            core.index, core.usage_percent, core.frequency_mhz, core.brand
        );
    }
    println!("Gouverneur de fréquence : {:?}", hardware.cpu.scaling_governor);
    println!("Vulnérabilités CPU :");
    for vulnerability in &hardware.cpu.vulnerabilities {
        println!("  {} : {}", vulnerability.name, vulnerability.status);
    }

    println!("=== Informations détaillées des stockages ===");
    for disk in &hardware.disks {
        println!("Nom :           {:?}", disk.name);
        println!("Type :          {}", disk.kind);
        println!("Système de FS : {}", disk.file_system);
        println!("Point de montage: {}", disk.mount_point);
        println!(
            "Amovible :      {}",
            if disk.is_removable { "Oui" } else { "Non" }
        );
        println!(
            "Espace :        {} Go utilisés / {} Go au total",
            disk.used_gb, disk.total_gb
        );
        println!("Santé SMART :   {:?}", disk.smart_health);
        println!("---------------------------------------------");
    }

    println!("=> Stockage virtuel (overlay, conteneurs...) : {}", hardware.virtual_disks.len());
    for disk in &hardware.virtual_disks {
        println!(
            "  {} | {} | {} | {} Go utilisés / {} Go au total",
            disk.name, disk.file_system, disk.mount_point, disk.used_gb, disk.total_gb
        );
    }

    println!("=> Réseaux :");
    for network in &hardware.network.interfaces {
        println!(
            "Interface: {} | Reçu : {} B | Émis : {} B",
            network.interface_name, network.received_bytes, network.transmitted_bytes
        );
    }
    println!("Passerelle par défaut : {:?}", hardware.network.default_gateway);
    println!("Serveurs DNS : {:?}", hardware.network.dns_servers);

    println!("=> Wi-Fi :");
    for wifi in &hardware.wifi {
        println!(
            "  SSID: {} | Signal: {:?}% | Interface: {:?}",
            wifi.ssid, wifi.signal_percent, wifi.interface
        );
    }

    println!("=> Périphériques PCI : {}", hardware.pci_devices.len());
    for device in &hardware.pci_devices {
        println!("  {} [{}]", device.name, device.class);
    }

    println!("=> components:");
    for component in &hardware.components {
        println!(
            "  {}: {:?}°C (max: {:?}°C, critique: {:?}°C)",
            component.label,
            component.temperature_celsius,
            component.max_temperature_celsius,
            component.critical_temperature_celsius
        );
    }

    println!("=> Batterie(s) : {}", hardware.batteries.len());
    for battery in &hardware.batteries {
        println!(
            "  {} {} | État: {} | Charge: {:.1}% | Cycles: {:?}",
            battery.vendor.as_deref().unwrap_or("?"),
            battery.model.as_deref().unwrap_or("?"),
            battery.state,
            battery.state_of_charge_percent,
            battery.cycle_count
        );
    }

    println!("=> Carte mère / BIOS :");
    println!(
        "  Vendor: {:?} | Modèle: {:?} | Version: {:?}",
        hardware.motherboard.vendor, hardware.motherboard.model, hardware.motherboard.version
    );
    println!(
        "  BIOS Vendor: {:?} | BIOS Version: {:?} | BIOS Date: {:?}",
        hardware.motherboard.bios_vendor,
        hardware.motherboard.bios_version,
        hardware.motherboard.bios_date
    );
    println!("  UUID machine: {:?}", hardware.motherboard.machine_uuid);
    println!("  Secure Boot: {:?}", hardware.motherboard.secure_boot);

    println!("=> GPU(s) : {}", hardware.gpus.len());
    for gpu in &hardware.gpus {
        println!("  {} ({:?})", gpu.name, gpu.vendor);
    }

    println!("=> Écran(s) : {}", hardware.monitors.len());
    for monitor in &hardware.monitors {
        println!(
            "  {} - {}x{} @ {:.0}Hz | primaire: {} | intégré: {}",
            monitor.name,
            monitor.width,
            monitor.height,
            monitor.frequency_hz,
            monitor.is_primary,
            monitor.is_builtin
        );
    }

    println!("=> Lecteur(s) optique(s)/disquette(s) : {}", hardware.optical_drives.len());
    for drive in &hardware.optical_drives {
        println!("  {} ({:?}) [{}]", drive.name, drive.vendor, drive.kind);
    }

    println!("=> Périphérique(s) : {}", hardware.peripherals.len());
    for peripheral in &hardware.peripherals {
        println!("  {} [{}]", peripheral.name, peripheral.kind);
    }

    println!("=> Souris : {}", hardware.mice.len());
    for mouse in &hardware.mice {
        println!("  {}", mouse.name);
    }

    println!("=> Manette(s) : {}", hardware.gamepads.len());
    for gamepad in &hardware.gamepads {
        println!("  {}", gamepad.name);
    }

    println!("=> Touchpad(s) : {}", hardware.touchpads.len());
    for touchpad in &hardware.touchpads {
        println!("  {}", touchpad.name);
    }

    println!("=> Caméra(s) : {}", hardware.cameras.len());
    for camera in &hardware.cameras {
        println!("  {}", camera.name);
    }

    println!("=> Périphérique(s) externe(s) USB : {}", hardware.usb_devices.len());
    for device in &hardware.usb_devices {
        println!("  {} ({:?})", device.name, device.vendor);
    }

    println!("=> Périphérique(s) Bluetooth appairé(s) : {}", hardware.bluetooth_devices.len());
    for device in &hardware.bluetooth_devices {
        println!("  {}", device.name);
    }

    println!("=> Imprimante(s)/Scanner(s) : {}", hardware.printers.len());
    for printer in &hardware.printers {
        println!("  {} [{}]", printer.name, printer.kind);
    }

    println!("=> Ventilateur(s) : {}", hardware.fans.len());
    for fan in &hardware.fans {
        println!("  {} ({:?} tr/min)", fan.name, fan.speed_rpm);
    }
}

fn print_browsers(report: &SystemReport) {
    if report.browsers.is_empty() {
        println!("Aucun navigateur détecté.");
        return;
    }
    for browser in &report.browsers {
        println!(
            "{} | Version: {} | Par défaut: {} | Chemin: {}",
            browser.name,
            browser.version.as_deref().unwrap_or("inconnue"),
            if browser.is_default { "Oui" } else { "Non" },
            browser.path.as_deref().unwrap_or("?")
        );
    }
}
