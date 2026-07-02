// On importe les structures nécessaires depuis sysinfo
use sysinfo::{Components, Disks, System, Networks};

fn main() {
    // Initialisation du système pour collecter les données.
    let mut sys = System::new_all();

    // Actualisation des informations.
    sys.refresh_all();

    println!("--- System Data ---");

    ////////////////////////////
    // SOFTWARE
    ////////////////////////////

    // Système d'exploitation.
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // Temps d'allumage de l'ordinateur (Correction syntaxe associée).
    println!("Ordinateur allumé depuis : {} secondes", System::uptime());

    // Nombre de Processus en cours.
    println!("Nombre de processus en cours : {}", sys.processes().len());

    println!("=> Processus consommant plus de 5 % :");
    for (pid, process) in sys.processes() {
        // Filtrer par exemple les processus gourmands
        if process.cpu_usage() > 5.0 { 
            println!(
                "PID: {} | Nom: {} | CPU: {:.2}% | Mémoire: {} Mo",
                pid,
                process.name().to_string_lossy(),
                process.cpu_usage(),
                process.memory() / 1024 / 1024
            );
        }
    }

    // Initialisation et rafraîchissement des réseaux.
    let networks = Networks::new_with_refreshed_list();

    println!("=> Réseaux :");
    for (interface_name, data) in &networks {
        println!(
            "Interface: {} | Reçu : {} B | Émis : {} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }
    ////////////////////////////
    // HARDWARE
    ////////////////////////////

    // Mémoire.
    println!("total memory: {} Mo", sys.total_memory() / 1024 / 1024);
    println!("used memory : {} Mo", sys.used_memory() / 1024 / 1024);

    // Processeurs (Correction du format d'affichage `{:?}`).
    println!("List CPUs : {:?}", sys.cpus());
    println!("NB CPUs: {}", sys.cpus().len());

    // Disques durs et SSD.

    let disks = Disks::new_with_refreshed_list();
    println!("=== Informations détaillées des stockages ===");

    for disk in &disks {
        // Conversion des octets en Gigaoctets (Go)
        let total_gb = disk.total_space() / 1024 / 1024 / 1024;
        let available_gb = disk.available_space() / 1024 / 1024 / 1024;
        let used_gb = total_gb - available_gb;

        println!("Nom :           {:?}", disk.name());
        println!("Type :          {:?}", disk.kind()); // HDD, SSD, or Unknown
        println!("Système de FS : {:?}", disk.file_system());
        println!("Point de montage: {:?}", disk.mount_point());
        println!("Amovible :      {}", if disk.is_removable() { "Oui" } else { "Non" });
        println!("Espace :        {} Go utilisés / {} Go au total", used_gb, total_gb);
        println!("---------------------------------------------");
    }

    // Température de la carte mère et des composants.
    let components = Components::new_with_refreshed_list();
    println!("=> components:");
    for component in &components {
        println!("  {component:?}");
    }

    sys.refresh_cpu_all(); 

    println!("Utilisation globale du CPU : {}%", sys.global_cpu_usage());

    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!(
            "Cœur #{} : {}% | Fréquence : {} MHz | Marque : {}",
            i,
            cpu.cpu_usage(),
            cpu.frequency(),
            cpu.brand()
        );
    }
}