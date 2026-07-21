import { renderHome } from "./home.js";
import { getLocale, setLocale } from "./i18n.js";

const root = document.querySelector("#app");
// Aligne le backend Rust sur la langue par défaut du frontend dès le démarrage.
setLocale(getLocale());
renderHome(root);
