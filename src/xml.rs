use crate::report::SystemReport;
use std::fmt::Write as _;

pub fn generate(report: &SystemReport) -> String {
    let mut xml = String::new();

    writeln!(xml, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>").unwrap();
    writeln!(xml, "<system_report>").unwrap();
    writeln!(xml, "<generated_at_unix>{}</generated_at_unix>", report.generated_at_unix).unwrap();
    writeln!(xml, "<tool_version>{}</tool_version>", esc(&report.tool_version)).unwrap();

    write_software(&mut xml, report);
    write_hardware(&mut xml, report);
    write_browsers(&mut xml, report);
    write_warnings(&mut xml, report);
    write_processes(&mut xml, report);

    writeln!(xml, "</system_report>").unwrap();

    xml
}

fn simple_list_section<T>(xml: &mut String, tag: &str, item_tag: &str, items: &[T], row: impl Fn(&T) -> String) {
    writeln!(xml, "<{tag} count=\"{}\">", items.len()).unwrap();
    for item in items {
        writeln!(xml, "<{item_tag}>{}</{item_tag}>", row(item)).unwrap();
    }
    writeln!(xml, "</{tag}>").unwrap();
}

fn write_software(xml: &mut String, report: &SystemReport) {
    let software = &report.software;
    let os = &software.os;

    writeln!(xml, "<software>").unwrap();

    writeln!(xml, "<operating_system>").unwrap();
    writeln!(xml, "<name>{}</name>", opt(&os.name)).unwrap();
    writeln!(xml, "<kernel_version>{}</kernel_version>", opt(&os.kernel_version)).unwrap();
    writeln!(xml, "<os_version>{}</os_version>", opt(&os.os_version)).unwrap();
    writeln!(xml, "<host_name>{}</host_name>", opt(&os.host_name)).unwrap();
    writeln!(xml, "<uptime_seconds>{}</uptime_seconds>", os.uptime_seconds).unwrap();
    writeln!(xml, "</operating_system>").unwrap();

    simple_list_section(
        xml,
        "users",
        "user",
        &software.users,
        |u| {
            format!(
                "<name>{}</name><uid>{}</uid><gid>{}</gid><groups>{}</groups>",
                esc(&u.name), u.uid, u.gid, esc(&u.groups.join(", "))
            )
        },
    );

    writeln!(xml, "<environment_variables count=\"{}\">", software.env_vars.len()).unwrap();
    for env_var in &software.env_vars {
        writeln!(
            xml,
            "<variable><key>{}</key><value>{}</value></variable>",
            esc(&env_var.key), esc(&env_var.value)
        )
        .unwrap();
    }
    writeln!(xml, "</environment_variables>").unwrap();

    simple_list_section(
        xml,
        "installed_apps",
        "app",
        &software.installed_apps,
        |app| {
            format!(
                "<name>{}</name><version>{}</version><publisher>{}</publisher><source>{}</source>",
                esc(&app.name),
                esc(app.version.as_deref().unwrap_or("?")),
                esc(app.publisher.as_deref().unwrap_or("?")),
                esc(&app.source)
            )
        },
    );

    simple_list_section(
        xml,
        "dev_runtimes",
        "runtime",
        &software.dev_runtimes,
        |r| format!("<name>{}</name><version>{}</version>", esc(&r.name), esc(&r.version)),
    );

    simple_list_section(
        xml,
        "services",
        "service",
        &software.services,
        |s| format!("<name>{}</name><status>{}</status>", esc(&s.name), esc(&s.status)),
    );

    simple_list_section(
        xml,
        "failed_services",
        "service",
        &software.failed_services,
        |s| format!("<name>{}</name><status>{}</status>", esc(&s.name), esc(&s.status)),
    );

    simple_list_section(
        xml,
        "scheduled_tasks",
        "task",
        &software.scheduled_tasks,
        |t| format!("<name>{}</name><schedule>{}</schedule>", esc(&t.name), esc(&t.schedule)),
    );

    simple_list_section(
        xml,
        "autostart_entries",
        "entry",
        &software.autostart_entries,
        |e| {
            format!(
                "<name>{}</name><command>{}</command>",
                esc(&e.name), esc(e.command.as_deref().unwrap_or("?"))
            )
        },
    );

    simple_list_section(
        xml,
        "package_managers",
        "package_manager",
        &software.package_managers,
        |p| format!("<manager>{}</manager><package_count>{}</package_count>", esc(&p.manager), p.package_count),
    );

    simple_list_section(
        xml,
        "network_connections",
        "connection",
        &software.network_connections,
        |c| {
            format!(
                "<protocol>{}</protocol><local_address>{}</local_address><state>{}</state>",
                esc(&c.protocol), esc(&c.local_address), esc(&c.state)
            )
        },
    );

    simple_list_section(
        xml,
        "docker_images",
        "image",
        &software.docker_images,
        |img| {
            format!(
                "<repository>{}</repository><tag>{}</tag><image_id>{}</image_id><size>{}</size><created>{}</created>",
                esc(&img.repository), esc(&img.tag), esc(&img.image_id), esc(&img.size), esc(&img.created)
            )
        },
    );

    simple_list_section(
        xml,
        "docker_volumes",
        "volume",
        &software.docker_volumes,
        |v| {
            format!(
                "<name>{}</name><driver>{}</driver><mountpoint>{}</mountpoint>",
                esc(&v.name), esc(&v.driver), opt(&v.mountpoint)
            )
        },
    );

    simple_list_section(
        xml,
        "virtual_machines",
        "vm",
        &software.virtual_machines,
        |vm| {
            format!(
                "<name>{}</name><hypervisor>{}</hypervisor><state>{}</state><identifier>{}</identifier>",
                esc(&vm.name), esc(&vm.hypervisor), esc(&vm.state), opt(&vm.identifier)
            )
        },
    );

    simple_list_section(
        xml,
        "podman_images",
        "image",
        &software.podman_images,
        |img| {
            format!(
                "<repository>{}</repository><tag>{}</tag><image_id>{}</image_id><size>{}</size><created>{}</created>",
                esc(&img.repository), esc(&img.tag), esc(&img.image_id), esc(&img.size), esc(&img.created)
            )
        },
    );

    simple_list_section(
        xml,
        "podman_volumes",
        "volume",
        &software.podman_volumes,
        |v| {
            format!(
                "<name>{}</name><driver>{}</driver><mountpoint>{}</mountpoint>",
                esc(&v.name), esc(&v.driver), opt(&v.mountpoint)
            )
        },
    );

    simple_list_section(
        xml,
        "ssh_keys",
        "key",
        &software.ssh_keys,
        |k| {
            format!(
                "<file_name>{}</file_name><key_type>{}</key_type><fingerprint>{}</fingerprint>",
                esc(&k.file_name), opt(&k.key_type), opt(&k.fingerprint)
            )
        },
    );

    writeln!(xml, "<proxy_config>").unwrap();
    match &software.proxy_config {
        Some(proxy) => {
            writeln!(xml, "<http_proxy>{}</http_proxy>", opt(&proxy.http_proxy)).unwrap();
            writeln!(xml, "<https_proxy>{}</https_proxy>", opt(&proxy.https_proxy)).unwrap();
            writeln!(xml, "<no_proxy>{}</no_proxy>", opt(&proxy.no_proxy)).unwrap();
            writeln!(xml, "<source>{}</source>", esc(&proxy.source)).unwrap();
        }
        None => {}
    }
    writeln!(xml, "</proxy_config>").unwrap();

    writeln!(xml, "<fonts total_count=\"{}\">", software.fonts.total_count).unwrap();
    if !software.fonts.families.is_empty() {
        writeln!(xml, "<families>{}</families>", esc(&software.fonts.families.join(", "))).unwrap();
    }
    writeln!(xml, "</fonts>").unwrap();

    writeln!(xml, "<desktop_environment>").unwrap();
    writeln!(xml, "<desktop>{}</desktop>", opt(&software.desktop_environment.desktop)).unwrap();
    writeln!(xml, "<session_type>{}</session_type>", opt(&software.desktop_environment.session_type)).unwrap();
    writeln!(xml, "<locale>{}</locale>", opt(&software.desktop_environment.locale)).unwrap();
    writeln!(xml, "<timezone>{}</timezone>", opt(&software.desktop_environment.timezone)).unwrap();
    writeln!(xml, "</desktop_environment>").unwrap();

    simple_list_section(
        xml,
        "update_history",
        "update",
        &software.update_history,
        |u| format!("<date>{}</date><description>{}</description>", esc(&u.date), esc(&u.description)),
    );

    simple_list_section(
        xml,
        "kernel_modules",
        "module",
        &software.kernel_modules,
        |m| format!("<name>{}</name><size_bytes>{}</size_bytes>", esc(&m.name), m.size_bytes),
    );

    writeln!(xml, "</software>").unwrap();
}

fn write_hardware(xml: &mut String, report: &SystemReport) {
    let hardware = &report.hardware;

    writeln!(xml, "<hardware>").unwrap();

    writeln!(xml, "<memory>").unwrap();
    writeln!(xml, "<total_mb>{}</total_mb>", hardware.memory.total_mb).unwrap();
    writeln!(xml, "<used_mb>{}</used_mb>", hardware.memory.used_mb).unwrap();
    writeln!(xml, "<total_swap_mb>{}</total_swap_mb>", hardware.memory.total_swap_mb).unwrap();
    writeln!(xml, "<used_swap_mb>{}</used_swap_mb>", hardware.memory.used_swap_mb).unwrap();
    writeln!(xml, "</memory>").unwrap();

    writeln!(xml, "<cpu>").unwrap();
    writeln!(xml, "<architecture>{}</architecture>", esc(&hardware.cpu.architecture)).unwrap();
    writeln!(xml, "<core_count>{}</core_count>", hardware.cpu.core_count).unwrap();
    writeln!(xml, "<global_usage_percent>{:.1}</global_usage_percent>", hardware.cpu.global_usage_percent).unwrap();
    writeln!(xml, "<cores count=\"{}\">", hardware.cpu.cores.len()).unwrap();
    for core in &hardware.cpu.cores {
        writeln!(
            xml,
            "<core><index>{}</index><usage_percent>{:.1}</usage_percent><frequency_mhz>{}</frequency_mhz><brand>{}</brand></core>",
            core.index, core.usage_percent, core.frequency_mhz, esc(&core.brand)
        )
        .unwrap();
    }
    writeln!(xml, "</cores>").unwrap();
    writeln!(xml, "<scaling_governor>{}</scaling_governor>", opt(&hardware.cpu.scaling_governor)).unwrap();
    simple_list_section(
        xml,
        "vulnerabilities",
        "vulnerability",
        &hardware.cpu.vulnerabilities,
        |v| format!("<name>{}</name><status>{}</status>", esc(&v.name), esc(&v.status)),
    );
    writeln!(xml, "</cpu>").unwrap();

    simple_list_section(
        xml,
        "disks",
        "disk",
        &hardware.disks,
        |disk| {
            format!(
                "<name>{}</name><kind>{}</kind><file_system>{}</file_system><mount_point>{}</mount_point><is_removable>{}</is_removable><used_gb>{}</used_gb><total_gb>{}</total_gb><smart_health>{}</smart_health>",
                esc(&disk.name),
                esc(&disk.kind),
                esc(&disk.file_system),
                esc(&disk.mount_point),
                disk.is_removable,
                disk.used_gb,
                disk.total_gb,
                opt(&disk.smart_health)
            )
        },
    );

    simple_list_section(
        xml,
        "virtual_disks",
        "disk",
        &hardware.virtual_disks,
        |disk| {
            format!(
                "<name>{}</name><file_system>{}</file_system><mount_point>{}</mount_point>",
                esc(&disk.name), esc(&disk.file_system), esc(&disk.mount_point)
            )
        },
    );

    writeln!(xml, "<network>").unwrap();
    writeln!(xml, "<interfaces count=\"{}\">", hardware.network.interfaces.len()).unwrap();
    for network in &hardware.network.interfaces {
        writeln!(
            xml,
            "<interface><name>{}</name><received_bytes>{}</received_bytes><transmitted_bytes>{}</transmitted_bytes><mac_address>{}</mac_address><ipv4_addresses>{}</ipv4_addresses><ipv6_addresses>{}</ipv6_addresses><link_speed_mbps>{}</link_speed_mbps><connection_type>{}</connection_type></interface>",
            esc(&network.interface_name),
            network.received_bytes,
            network.transmitted_bytes,
            opt(&network.mac_address),
            esc(&network.ipv4_addresses.join(", ")),
            esc(&network.ipv6_addresses.join(", ")),
            network.link_speed_mbps.map(|v| v.to_string()).unwrap_or_else(|| "?".to_string()),
            opt(&network.connection_type),
        )
        .unwrap();
    }
    writeln!(xml, "</interfaces>").unwrap();
    writeln!(xml, "<default_gateway>{}</default_gateway>", opt(&hardware.network.default_gateway)).unwrap();
    writeln!(
        xml,
        "<dns_servers>{}</dns_servers>",
        esc(&hardware.network.dns_servers.join(", "))
    )
    .unwrap();
    writeln!(xml, "</network>").unwrap();

    simple_list_section(
        xml,
        "wifi",
        "network",
        &hardware.wifi,
        |w| {
            format!(
                "<ssid>{}</ssid><signal_percent>{}</signal_percent><interface>{}</interface>",
                esc(&w.ssid),
                w.signal_percent.map(|s| s.to_string()).unwrap_or_else(|| "?".to_string()),
                esc(w.interface.as_deref().unwrap_or("?"))
            )
        },
    );

    simple_list_section(
        xml,
        "pci_devices",
        "device",
        &hardware.pci_devices,
        |p| format!("<name>{}</name><class>{}</class>", esc(&p.name), esc(&p.class)),
    );

    simple_list_section(
        xml,
        "components",
        "component",
        &hardware.components,
        |component| {
            format!(
                "<label>{}</label><temperature_celsius>{}</temperature_celsius><max_temperature_celsius>{}</max_temperature_celsius><critical_temperature_celsius>{}</critical_temperature_celsius>",
                esc(&component.label),
                opt_num(component.temperature_celsius),
                opt_num(component.max_temperature_celsius),
                opt_num(component.critical_temperature_celsius)
            )
        },
    );

    simple_list_section(
        xml,
        "batteries",
        "battery",
        &hardware.batteries,
        |battery| {
            format!(
                "<vendor>{}</vendor><model>{}</model><state>{}</state><technology>{}</technology><state_of_charge_percent>{:.1}</state_of_charge_percent><state_of_health_percent>{:.1}</state_of_health_percent><cycle_count>{}</cycle_count>",
                esc(battery.vendor.as_deref().unwrap_or("?")),
                esc(battery.model.as_deref().unwrap_or("?")),
                esc(&battery.state),
                esc(&battery.technology),
                battery.state_of_charge_percent,
                battery.state_of_health_percent,
                opt_num(battery.cycle_count.map(|c| c as f32))
            )
        },
    );

    writeln!(xml, "<motherboard>").unwrap();
    writeln!(xml, "<vendor>{}</vendor>", opt(&hardware.motherboard.vendor)).unwrap();
    writeln!(xml, "<model>{}</model>", opt(&hardware.motherboard.model)).unwrap();
    writeln!(xml, "<version>{}</version>", opt(&hardware.motherboard.version)).unwrap();
    writeln!(xml, "<bios_vendor>{}</bios_vendor>", opt(&hardware.motherboard.bios_vendor)).unwrap();
    writeln!(xml, "<bios_version>{}</bios_version>", opt(&hardware.motherboard.bios_version)).unwrap();
    writeln!(xml, "<bios_date>{}</bios_date>", opt(&hardware.motherboard.bios_date)).unwrap();
    writeln!(xml, "<machine_uuid>{}</machine_uuid>", opt(&hardware.motherboard.machine_uuid)).unwrap();
    writeln!(xml, "<secure_boot>{}</secure_boot>", opt(&hardware.motherboard.secure_boot)).unwrap();
    writeln!(xml, "</motherboard>").unwrap();

    simple_list_section(
        xml,
        "gpus",
        "gpu",
        &hardware.gpus,
        |gpu| {
            format!(
                "<name>{}</name><vendor>{}</vendor><vram_mb>{}</vram_mb><driver_version>{}</driver_version>",
                esc(&gpu.name),
                opt(&gpu.vendor),
                gpu.vram_mb.map(|v| v.to_string()).unwrap_or_else(|| "?".to_string()),
                opt(&gpu.driver_version)
            )
        },
    );

    simple_list_section(
        xml,
        "monitors",
        "monitor",
        &hardware.monitors,
        |monitor| {
            format!(
                "<name>{}</name><width>{}</width><height>{}</height><x>{}</x><y>{}</y><scale_factor>{:.2}</scale_factor><frequency_hz>{:.0}</frequency_hz><is_primary>{}</is_primary><is_builtin>{}</is_builtin>",
                esc(&monitor.name),
                monitor.width,
                monitor.height,
                monitor.x,
                monitor.y,
                monitor.scale_factor,
                monitor.frequency_hz,
                monitor.is_primary,
                monitor.is_builtin
            )
        },
    );

    simple_list_section(
        xml,
        "optical_drives",
        "drive",
        &hardware.optical_drives,
        |drive| {
            format!(
                "<name>{}</name><vendor>{}</vendor><kind>{}</kind>",
                esc(&drive.name), esc(drive.vendor.as_deref().unwrap_or("?")), esc(&drive.kind)
            )
        },
    );

    simple_list_section(
        xml,
        "peripherals",
        "peripheral",
        &hardware.peripherals,
        |peripheral| format!("<name>{}</name><kind>{}</kind>", esc(&peripheral.name), esc(&peripheral.kind)),
    );

    simple_list_section(
        xml,
        "mice",
        "mouse",
        &hardware.mice,
        |mouse| format!("<name>{}</name>", esc(&mouse.name)),
    );

    simple_list_section(
        xml,
        "gamepads",
        "gamepad",
        &hardware.gamepads,
        |gamepad| format!("<name>{}</name>", esc(&gamepad.name)),
    );

    simple_list_section(
        xml,
        "touchpads",
        "touchpad",
        &hardware.touchpads,
        |touchpad| format!("<name>{}</name>", esc(&touchpad.name)),
    );

    simple_list_section(
        xml,
        "cameras",
        "camera",
        &hardware.cameras,
        |camera| format!("<name>{}</name>", esc(&camera.name)),
    );

    simple_list_section(
        xml,
        "usb_devices",
        "device",
        &hardware.usb_devices,
        |device| format!("<name>{}</name><vendor>{}</vendor>", esc(&device.name), esc(device.vendor.as_deref().unwrap_or("?"))),
    );

    simple_list_section(
        xml,
        "bluetooth_devices",
        "device",
        &hardware.bluetooth_devices,
        |device| format!("<name>{}</name>", esc(&device.name)),
    );

    simple_list_section(
        xml,
        "printers",
        "printer",
        &hardware.printers,
        |printer| format!("<name>{}</name><kind>{}</kind>", esc(&printer.name), esc(&printer.kind)),
    );

    simple_list_section(
        xml,
        "fans",
        "fan",
        &hardware.fans,
        |fan| {
            format!(
                "<name>{}</name><speed_rpm>{}</speed_rpm>",
                esc(&fan.name),
                fan.speed_rpm.map(|v| v.to_string()).unwrap_or_else(|| "?".to_string())
            )
        },
    );

    simple_list_section(
        xml,
        "partitions",
        "partition",
        &hardware.storage_layout.partitions,
        |p| format!("<device>{}</device><fs_type>{}</fs_type><size_gb>{}</size_gb>", esc(&p.device), esc(&p.fs_type), p.size_gb),
    );

    simple_list_section(
        xml,
        "lvm_volumes",
        "volume",
        &hardware.storage_layout.lvm_volumes,
        |v| format!("<vg_name>{}</vg_name><lv_name>{}</lv_name><size_gb>{}</size_gb>", esc(&v.vg_name), esc(&v.lv_name), v.size_gb),
    );

    simple_list_section(
        xml,
        "raid_arrays",
        "array",
        &hardware.storage_layout.raid_arrays,
        |r| {
            format!(
                "<device>{}</device><level>{}</level><state>{}</state><devices>{}</devices>",
                esc(&r.device), esc(&r.level), esc(&r.state), esc(&r.devices.join(", "))
            )
        },
    );

    writeln!(xml, "<power_profile>").unwrap();
    writeln!(xml, "<profile>{}</profile>", opt(&hardware.power_profile.profile)).unwrap();
    writeln!(xml, "<sleep_mode>{}</sleep_mode>", opt(&hardware.power_profile.sleep_mode)).unwrap();
    writeln!(xml, "</power_profile>").unwrap();

    writeln!(xml, "</hardware>").unwrap();
}

fn write_browsers(xml: &mut String, report: &SystemReport) {
    simple_list_section(
        xml,
        "browsers",
        "browser",
        &report.browsers,
        |browser| {
            format!(
                "<name>{}</name><version>{}</version><is_default>{}</is_default><path>{}</path>",
                esc(&browser.name),
                esc(browser.version.as_deref().unwrap_or("inconnue")),
                browser.is_default,
                esc(browser.path.as_deref().unwrap_or("?"))
            )
        },
    );
}

fn write_warnings(xml: &mut String, report: &SystemReport) {
    simple_list_section(
        xml,
        "collection_warnings",
        "warning",
        &report.collection_warnings,
        |warning| esc(warning),
    );
}

fn write_processes(xml: &mut String, report: &SystemReport) {
    let processes = &report.software.processes;

    writeln!(xml, "<processes total_count=\"{}\">", processes.total_count).unwrap();
    for process in &processes.processes {
        writeln!(
            xml,
            "<process><pid>{}</pid><name>{}</name><cpu_usage_percent>{:.2}</cpu_usage_percent><memory_mb>{}</memory_mb></process>",
            process.pid, esc(&process.name), process.cpu_usage_percent, process.memory_mb
        )
        .unwrap();
    }
    writeln!(xml, "</processes>").unwrap();
}

fn opt(value: &Option<String>) -> String {
    esc(value.as_deref().unwrap_or("?"))
}

fn opt_num(value: Option<f32>) -> String {
    value.map(|v| format!("{v:.1}")).unwrap_or_else(|| "?".to_string())
}

fn esc(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
