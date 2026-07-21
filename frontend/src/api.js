import { invoke } from "@tauri-apps/api/core";

export const getConsent = () => invoke("get_consent");
export const saveConsent = (config) => invoke("save_consent", { config });
export const collectAndExport = (formats, outputDir) =>
  invoke("collect_and_export", { formats, outputDir });
