import { invoke } from "@tauri-apps/api/core";

export const setLocale = (locale) => invoke("set_locale", { locale });
export const getConsent = () => invoke("get_consent");
export const saveConsent = (config) => invoke("save_consent", { config });
export const getPreset = (name) => invoke("get_preset", { name });
export const listHardwareFields = () => invoke("list_hardware_fields");
export const listSoftwareFields = () => invoke("list_software_fields");
export const collectAndExport = (formats, outputDir) =>
  invoke("collect_and_export", { formats, outputDir });
export const getRemoteExportConfig = () => invoke("get_remote_export_config");
export const saveRemoteExportConfig = (config) =>
  invoke("save_remote_export_config", { config });
export const listSnapshots = () => invoke("list_snapshots");
export const openStorageLocation = () => invoke("open_storage_location");
