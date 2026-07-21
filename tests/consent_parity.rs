use tracker::consent::ConsentConfig;
use tracker::hardware::HARDWARE_FIELDS;
use tracker::software::SOFTWARE_FIELDS;

fn sorted_object_keys(value: &serde_json::Value) -> Vec<String> {
    let mut keys: Vec<String> = value
        .as_object()
        .expect("valeur attendue comme objet JSON")
        .keys()
        .cloned()
        .collect();
    keys.sort();
    keys
}

fn sorted(fields: &[&str]) -> Vec<String> {
    let mut owned: Vec<String> = fields.iter().map(|s| s.to_string()).collect();
    owned.sort();
    owned
}

#[test]
fn hardware_consent_keys_match_hardware_info_fields() {
    let config = ConsentConfig::default();
    let value = serde_json::to_value(&config.hardware).unwrap();
    assert_eq!(
        sorted_object_keys(&value),
        sorted(HARDWARE_FIELDS),
        "ConsentConfig.hardware a divergé de hardware::HARDWARE_FIELDS — \
         mettre à jour consent::HardwareConsent, hardware::HARDWARE_FIELDS et \
         hardware::HardwareInfo en même temps."
    );
}

#[test]
fn software_consent_keys_match_software_info_fields() {
    let config = ConsentConfig::default();
    let value = serde_json::to_value(&config.software).unwrap();
    assert_eq!(
        sorted_object_keys(&value),
        sorted(SOFTWARE_FIELDS),
        "ConsentConfig.software a divergé de software::SOFTWARE_FIELDS — \
         mettre à jour consent::SoftwareConsent, software::SOFTWARE_FIELDS et \
         software::SoftwareInfo en même temps."
    );
}
