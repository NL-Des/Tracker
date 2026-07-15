pub struct EdidIds {
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
}

const EDID_MAGIC: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];

/// Décode l'ID fabricant PNP (3 lettres) encodé sur les octets 8-9 de l'en-tête
/// EDID : 5 bits par lettre (A=1..Z=26), lettre 'A' = 0x01 dans la base 26.
fn decode_pnp_id(b8: u8, b9: u8) -> Option<String> {
    let raw = ((b8 as u16) << 8) | b9 as u16;
    let c1 = ((raw >> 10) & 0x1F) as u8;
    let c2 = ((raw >> 5) & 0x1F) as u8;
    let c3 = (raw & 0x1F) as u8;
    let decode = |v: u8| -> Option<char> {
        if (1..=26).contains(&v) {
            Some((b'A' + v - 1) as char)
        } else {
            None
        }
    };
    Some(
        [decode(c1)?, decode(c2)?, decode(c3)?]
            .iter()
            .collect::<String>(),
    )
}

/// Cherche dans les 4 blocs descripteurs (18 octets chacun, à partir de
/// l'octet 54) un descripteur de type "nom du produit" (0xFC) ou "numéro de
/// série" (0xFF), reconnaissables par leur préfixe `00 00 00 <tag> 00`.
fn find_descriptor_text(edid: &[u8], tag: u8) -> Option<String> {
    for i in 0..4 {
        let offset = 54 + i * 18;
        let block = edid.get(offset..offset + 18)?;
        if block[0] == 0x00 && block[1] == 0x00 && block[2] == 0x00 && block[3] == tag {
            let text = &block[5..18];
            let text = String::from_utf8_lossy(text);
            let text = text.trim_end_matches(['\n', '\r', ' ', '\0']).to_string();
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

/// Parse un bloc EDID brut (128 octets minimum) pour en extraire l'ID
/// fabricant PNP, le nom de modèle et le numéro de série, quand ces
/// descripteurs sont présents (non garanti selon l'écran).
pub fn parse_edid(edid: &[u8]) -> Option<EdidIds> {
    if edid.len() < 128 || edid[0..8] != EDID_MAGIC {
        return None;
    }
    let vendor = decode_pnp_id(edid[8], edid[9]);
    let model = find_descriptor_text(edid, 0xFC);
    let serial_number = find_descriptor_text(edid, 0xFF);
    Some(EdidIds {
        vendor,
        model,
        serial_number,
    })
}
