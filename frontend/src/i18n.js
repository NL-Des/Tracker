import fr from "./locales/fr.json";
import en from "./locales/en.json";
import { setLocale as setBackendLocale } from "./api.js";

const dictionaries = { fr, en };
const fallbackLocale = "en";
let currentLocale = "fr";

export function t(key) {
  return dictionaries[currentLocale]?.[key] ?? dictionaries[fallbackLocale][key] ?? key;
}

export function getLocale() {
  return currentLocale;
}

// Bascule la langue côté frontend et notifie le backend Rust (collection_warnings, etc.)
// pour que les deux restent synchronisés.
export async function setLocale(locale) {
  if (!dictionaries[locale]) return;
  currentLocale = locale;
  await setBackendLocale(locale);
}
