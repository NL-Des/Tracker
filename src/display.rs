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
    println!("=> Processus consommant plus de 5 % :");
    for process in &software.processes.high_cpu_processes {
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
        println!("---------------------------------------------");
    }

    println!("=> Réseaux :");
    for network in &hardware.networks {
        println!(
            "Interface: {} | Reçu : {} B | Émis : {} B",
            network.interface_name, network.received_bytes, network.transmitted_bytes
        );
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

    println!("=> GPU(s) : {}", hardware.gpus.len());
    for gpu in &hardware.gpus {
        println!("  {} ({:?})", gpu.name, gpu.vendor);
    }

    println!("=> Écran(s) : {}", hardware.monitors.len());
    for monitor in &hardware.monitors {
        println!(
            "  {} - {}x{} @ {:.0}Hz | primaire: {}",
            monitor.name, monitor.width, monitor.height, monitor.frequency_hz, monitor.is_primary
        );
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
