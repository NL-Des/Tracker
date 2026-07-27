use crate::report::SystemReport;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const SCHEMA_VERSION: i64 = 1;

/// Résumé d'un snapshot historisé, tel qu'affiché dans une liste (sans le
/// JSON complet, trop volumineux pour un simple listing).
#[derive(Serialize)]
pub struct SnapshotSummary {
    pub id: i64,
    pub machine_id: Option<String>,
    pub collected_at_unix: i64,
    pub os_name: Option<String>,
    pub host_name: Option<String>,
}

pub fn db_path() -> std::io::Result<PathBuf> {
    directories::ProjectDirs::from("com", "tracker", "tracker")
        .map(|dirs| dirs.data_dir().join("tracker.db"))
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "répertoire de données utilisateur introuvable",
            )
        })
}

fn open_at(path: &Path) -> Result<Connection, String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let conn = Connection::open(path).map_err(|e| e.to_string())?;
    migrate(&conn)?;
    Ok(conn)
}

pub fn open() -> Result<Connection, String> {
    open_at(&db_path().map_err(|e| e.to_string())?)
}

fn migrate(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS snapshots (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            machine_id      TEXT,
            collected_at_unix INTEGER NOT NULL,
            schema_version  INTEGER NOT NULL,
            raw_json        TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS hardware_summary (
            snapshot_id     INTEGER PRIMARY KEY REFERENCES snapshots(id),
            cpu_architecture TEXT,
            cpu_core_count  INTEGER,
            ram_total_mb    INTEGER,
            disk_total_gb   INTEGER
        );

        CREATE TABLE IF NOT EXISTS software_summary (
            snapshot_id     INTEGER PRIMARY KEY REFERENCES snapshots(id),
            os_name         TEXT,
            os_version      TEXT,
            host_name       TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_snapshots_machine_collected
            ON snapshots(machine_id, collected_at_unix);
        ",
    )
    .map_err(|e| e.to_string())
}

/// Insère un snapshot complet (JSON brut + colonnes résumées pour le
/// requêtage SQL) et renvoie l'id de la ligne créée dans `snapshots`.
pub fn insert_snapshot(conn: &Connection, report: &SystemReport) -> Result<i64, String> {
    let raw_json = report.to_json_pretty().map_err(|e| e.to_string())?;
    let machine_id = report
        .hardware
        .motherboard
        .machine_uuid
        .clone()
        .or_else(|| report.software.os.host_name.clone());
    let collected_at_unix = report.generated_at_unix as i64;

    conn.execute(
        "INSERT INTO snapshots (machine_id, collected_at_unix, schema_version, raw_json)
         VALUES (?1, ?2, ?3, ?4)",
        params![machine_id, collected_at_unix, SCHEMA_VERSION, raw_json],
    )
    .map_err(|e| e.to_string())?;
    let snapshot_id = conn.last_insert_rowid();

    let disk_total_gb: u64 = report.hardware.disks.iter().map(|d| d.total_gb).sum();
    conn.execute(
        "INSERT INTO hardware_summary (snapshot_id, cpu_architecture, cpu_core_count, ram_total_mb, disk_total_gb)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            snapshot_id,
            report.hardware.cpu.architecture,
            report.hardware.cpu.core_count as i64,
            report.hardware.memory.total_mb as i64,
            disk_total_gb as i64,
        ],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO software_summary (snapshot_id, os_name, os_version, host_name)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            snapshot_id,
            report.software.os.name,
            report.software.os.os_version,
            report.software.os.host_name,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(snapshot_id)
}

/// Insère un snapshot en ouvrant une connexion à la base par défaut
/// (`db_path()`). Pratique pour les appelants qui n'ont pas déjà de
/// connexion ouverte (CLI, commande Tauri).
pub fn record_snapshot(report: &SystemReport) -> Result<i64, String> {
    let conn = open()?;
    insert_snapshot(&conn, report)
}

pub fn list_snapshots(conn: &Connection) -> Result<Vec<SnapshotSummary>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT s.id, s.machine_id, s.collected_at_unix, sw.os_name, sw.host_name
             FROM snapshots s
             LEFT JOIN software_summary sw ON sw.snapshot_id = s.id
             ORDER BY s.collected_at_unix DESC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(SnapshotSummary {
                id: row.get(0)?,
                machine_id: row.get(1)?,
                collected_at_unix: row.get(2)?,
                os_name: row.get(3)?,
                host_name: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

/// Renvoie le JSON brut complet d'un snapshot par son id.
pub fn get_snapshot_json(conn: &Connection, id: i64) -> Result<Option<String>, String> {
    conn.query_row(
        "SELECT raw_json FROM snapshots WHERE id = ?1",
        params![id],
        |row| row.get(0),
    )
    .map(Some)
    .or_else(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => Ok(None),
        other => Err(other.to_string()),
    })
}

#[allow(dead_code)]
fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn report_fixture() -> SystemReport {
        SystemReport::collect()
    }

    #[test]
    fn insert_and_list_round_trip() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        let report = report_fixture();

        let id = insert_snapshot(&conn, &report).unwrap();
        let summaries = list_snapshots(&conn).unwrap();

        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].id, id);
        assert_eq!(summaries[0].host_name, report.software.os.host_name);
    }

    #[test]
    fn get_snapshot_json_returns_full_report() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        let report = report_fixture();
        let id = insert_snapshot(&conn, &report).unwrap();

        let json = get_snapshot_json(&conn, id).unwrap().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["tool_version"], report.tool_version);
    }

    #[test]
    fn get_snapshot_json_missing_id_returns_none() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        assert_eq!(get_snapshot_json(&conn, 999).unwrap(), None);
    }

    #[test]
    fn multiple_snapshots_are_ordered_most_recent_first() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        let mut report = report_fixture();

        report.generated_at_unix = 100;
        insert_snapshot(&conn, &report).unwrap();
        report.generated_at_unix = 200;
        let second_id = insert_snapshot(&conn, &report).unwrap();

        let summaries = list_snapshots(&conn).unwrap();
        assert_eq!(summaries[0].id, second_id);
    }
}
