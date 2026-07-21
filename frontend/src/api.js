import { invoke } from "@tauri-apps/api/core";

export const setLocale = (locale) => invoke("set_locale", { locale });
export const getConsent = () => invoke("get_consent");
export const saveConsent = (config) => invoke("save_consent", { config });
export const getPreset = (name) => invoke("get_preset", { name });
export const listHardwareFields = () => invoke("list_hardware_fields");
export const collectAndExport = (formats, outputDir) =>
  invoke("collect_and_export", { formats, outputDir });
