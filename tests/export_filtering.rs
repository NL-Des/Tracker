use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::OnceLock;
use tracker::consent::{ConsentConfig, ConsentPreset};
use tracker::hardware::HARDWARE_FIELDS;
use tracker::remote_export::{send_report, RemoteExportConfig};
use tracker::software::SOFTWARE_FIELDS;
use tracker::{markdown, xml, SystemReport};

// `SystemReport::collect()` fait un vrai tour de collecte système (avec sleep
// CPU) : on ne l'appelle qu'une fois pour tout le fichier de test.
fn report() -> &'static SystemReport {
    static REPORT: OnceLock<SystemReport> = OnceLock::new();
    REPORT.get_or_init(SystemReport::collect)
}

fn with_hardware_field_disabled(field: &str) -> ConsentConfig {
    let mut consent = ConsentPreset::Maximum.to_config();
    let mut hw_value = serde_json::to_value(&consent.hardware).unwrap();
    hw_value[field] = serde_json::Value::Bool(false);
    consent.hardware = serde_json::from_value(hw_value).unwrap();
    consent
}

fn with_software_field_disabled(field: &str) -> ConsentConfig {
    let mut consent = ConsentPreset::Maximum.to_config();
    let mut sw_value = serde_json::to_value(&consent.software).unwrap();
    sw_value[field] = serde_json::Value::Bool(false);
    consent.software = serde_json::from_value(sw_value).unwrap();
    consent
}

#[test]
fn json_filtering_replaces_each_disabled_hardware_field_with_np() {
    let report = report();
    for &field in HARDWARE_FIELDS {
        let consent = with_hardware_field_disabled(field);
        let json = report.to_json_pretty_filtered(&consent).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(
            value["hardware"][field],
            serde_json::Value::String("np".to_string()),
            "hardware.{field} désactivé devrait être remplacé par \"np\""
        );
        let sibling = HARDWARE_FIELDS.iter().find(|f| **f != field).unwrap();
        assert_ne!(
            value["hardware"][sibling],
            serde_json::Value::String("np".to_string()),
            "hardware.{sibling} est resté activé, il ne doit pas être filtré"
        );
    }
}

#[test]
fn json_filtering_replaces_each_disabled_software_field_with_np() {
    let report = report();
    for &field in SOFTWARE_FIELDS {
        let consent = with_software_field_disabled(field);
        let json = report.to_json_pretty_filtered(&consent).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(
            value["software"][field],
            serde_json::Value::String("np".to_string()),
            "software.{field} désactivé devrait être remplacé par \"np\""
        );
        let sibling = SOFTWARE_FIELDS.iter().find(|f| **f != field).unwrap();
        assert_ne!(
            value["software"][sibling],
            serde_json::Value::String("np".to_string()),
            "software.{sibling} est resté activé, il ne doit pas être filtré"
        );
    }
}

#[test]
fn json_filtering_replaces_browsers_with_np_when_disabled() {
    let report = report();
    let mut consent = ConsentPreset::Maximum.to_config();
    consent.browsers = false;
    let json = report.to_json_pretty_filtered(&consent).unwrap();
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(value["browsers"], serde_json::Value::String("np".to_string()));
}

#[test]
fn json_filtering_keeps_everything_full_when_all_enabled() {
    let report = report();
    let consent = ConsentPreset::Maximum.to_config();
    let json = report.to_json_pretty_filtered(&consent).unwrap();
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    for &field in HARDWARE_FIELDS {
        assert_ne!(value["hardware"][field], serde_json::Value::String("np".to_string()));
    }
    for &field in SOFTWARE_FIELDS {
        assert_ne!(value["software"][field], serde_json::Value::String("np".to_string()));
    }
    assert_ne!(value["browsers"], serde_json::Value::String("np".to_string()));
}

#[test]
fn markdown_filters_disabled_hardware_fields() {
    let report = report();

    let cpu_disabled = with_hardware_field_disabled("cpu");
    assert!(markdown::generate(report, &cpu_disabled).contains("### CPU\n\nnp\n"));

    let disks_disabled = with_hardware_field_disabled("disks");
    assert!(markdown::generate(report, &disks_disabled).contains("### Stockage\n\nnp\n"));

    let storage_layout_disabled = with_hardware_field_disabled("storage_layout");
    assert!(markdown::generate(report, &storage_layout_disabled)
        .contains("### Organisation du stockage (partitions / LVM / RAID)\n\nnp\n"));

    let enabled = markdown::generate(report, &ConsentPreset::Maximum.to_config());
    assert!(enabled.contains("### CPU"));
    assert!(!enabled.contains("### CPU\n\nnp\n"));
}

#[test]
fn markdown_filters_disabled_software_fields() {
    let report = report();

    let os_disabled = with_software_field_disabled("os");
    assert!(markdown::generate(report, &os_disabled).contains("### Système d'exploitation\n\nnp\n"));

    let apps_disabled = with_software_field_disabled("installed_apps");
    assert!(markdown::generate(report, &apps_disabled).contains("### Applications installées\n\nnp\n"));

    let mut processes_disabled = ConsentPreset::Maximum.to_config();
    processes_disabled.software.processes = false;
    assert!(markdown::generate(report, &processes_disabled).contains("## Processus\n\nnp\n"));
}

#[test]
fn markdown_filters_browsers_when_disabled() {
    let report = report();
    let mut consent = ConsentPreset::Maximum.to_config();
    consent.browsers = false;
    assert!(markdown::generate(report, &consent).contains("## Navigateurs\n\nnp\n"));
}

#[test]
fn xml_filters_disabled_hardware_fields() {
    let report = report();

    let cpu_disabled = with_hardware_field_disabled("cpu");
    assert!(xml::generate(report, &cpu_disabled).contains("<cpu>np</cpu>"));

    let disks_disabled = with_hardware_field_disabled("disks");
    assert!(xml::generate(report, &disks_disabled).contains("<disks>np</disks>"));

    let storage_layout_disabled = with_hardware_field_disabled("storage_layout");
    let xml_out = xml::generate(report, &storage_layout_disabled);
    assert!(xml_out.contains("<partitions>np</partitions>"));
    assert!(xml_out.contains("<lvm_volumes>np</lvm_volumes>"));
    assert!(xml_out.contains("<raid_arrays>np</raid_arrays>"));

    let enabled = xml::generate(report, &ConsentPreset::Maximum.to_config());
    assert!(!enabled.contains("<cpu>np</cpu>"));
    assert!(enabled.contains("<architecture>"));
}

#[test]
fn xml_filters_disabled_software_fields() {
    let report = report();

    let os_disabled = with_software_field_disabled("os");
    assert!(xml::generate(report, &os_disabled).contains("<operating_system>np</operating_system>"));

    let apps_disabled = with_software_field_disabled("installed_apps");
    assert!(xml::generate(report, &apps_disabled).contains("<installed_apps>np</installed_apps>"));

    let mut processes_disabled = ConsentPreset::Maximum.to_config();
    processes_disabled.software.processes = false;
    assert!(xml::generate(report, &processes_disabled).contains("<processes>np</processes>"));
}

#[test]
fn xml_filters_browsers_when_disabled() {
    let report = report();
    let mut consent = ConsentPreset::Maximum.to_config();
    consent.browsers = false;
    assert!(xml::generate(report, &consent).contains("<browsers>np</browsers>"));
}

// Serveur HTTP jetable in-process (mêmes contraintes que
// `remote_export::tests` : pas de nouvelle dépendance de mock HTTP).
fn spawn_one_shot_server() -> (String, std::thread::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let handle = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();

        let mut content_length = 0usize;
        loop {
            let mut header_line = String::new();
            reader.read_line(&mut header_line).unwrap();
            if header_line == "\r\n" || header_line.is_empty() {
                break;
            }
            if let Some(v) = header_line.to_lowercase().strip_prefix("content-length:") {
                content_length = v.trim().parse().unwrap_or(0);
            }
        }
        let mut body = vec![0u8; content_length];
        reader.read_exact(&mut body).unwrap();

        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n")
            .unwrap();
        String::from_utf8_lossy(&body).to_string()
    });
    (format!("http://{addr}"), handle)
}

#[test]
fn remote_export_sends_the_same_filtered_json_as_file_exports() {
    let report = report();
    let consent = with_hardware_field_disabled("cpu");
    let expected_json = report.to_json_pretty_filtered(&consent).unwrap();

    let (url, handle) = spawn_one_shot_server();
    let config = RemoteExportConfig { enabled: true, url, auth_token: None };
    send_report(&config, &expected_json).unwrap();
    let received_body = handle.join().unwrap();

    let value: serde_json::Value = serde_json::from_str(&received_body).unwrap();
    assert_eq!(
        value["hardware"]["cpu"],
        serde_json::Value::String("np".to_string()),
        "le rapport envoyé au serveur distant doit respecter le même filtrage \"np\" que les exports fichiers"
    );
    assert_eq!(received_body, expected_json);
}
