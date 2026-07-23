import { renderApp } from "./app.js";
import { getConsent } from "./api.js";
import { setConsentState } from "./state.js";
import { getLocale, setLocale } from "./i18n.js";

const root = document.querySelector("#app");
// Aligne le backend Rust sur la langue par défaut du frontend dès le démarrage.
setLocale(getLocale());
// Chargé une seule fois ici (et non plus derrière un bouton "Commencer") car les
// onglets Hardware/Software sont montés immédiatement et lisent l'état au montage.
const consent = await getConsent();
setConsentState(consent);
renderApp(root);
