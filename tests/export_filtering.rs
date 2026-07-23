use std::sync::OnceLock;
use tracker::consent::{ConsentConfig, ConsentPreset};
use tracker::hardware::HARDWARE_FIELDS;
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
